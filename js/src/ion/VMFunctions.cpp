/* -*- Mode: C++; tab-width: 4; indent-tabs-mode: nil; c-basic-offset: 4 -*-
 * vim: set ts=4 sw=4 et tw=99:
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#include "Ion.h"
#include "IonCompartment.h"
#include "jsinterp.h"
#include "ion/BaselineIC.h"
#include "ion/IonFrames.h"
#include "ion/IonFrames-inl.h" // for GetTopIonJSScript

#include "vm/StringObject-inl.h"
#include "vm/Debugger.h"

#include "jsboolinlines.h"
#include "jsinterpinlines.h"

using namespace js;
using namespace js::ion;

namespace js {
namespace ion {

// Don't explicitly initialize, it's not guaranteed that this initializer will
// run before the constructors for static VMFunctions.
/* static */ VMFunction *VMFunction::functions;

void
VMFunction::addToFunctions()
{
    static bool initialized = false;
    if (!initialized) {
        initialized = true;
        functions = NULL;
    }
    this->next = functions;
    functions = this;
}

static inline bool
ShouldMonitorReturnType(JSFunction *fun)
{
    return fun->isInterpreted() &&
           (!fun->nonLazyScript()->hasAnalysis() ||
            !fun->nonLazyScript()->analysis()->ranInference());
}

bool
InvokeFunction(JSContext *cx, HandleFunction fun0, uint32_t argc, Value *argv, Value *rval)
{
    AssertCanGC();

    RootedFunction fun(cx, fun0);

    // In order to prevent massive bouncing between Ion and JM, see if we keep
    // hitting functions that are uncompilable.
    if (fun->isInterpreted()) {
        if (fun->isInterpretedLazy() && !JSFunction::getOrCreateScript(cx, fun))
            return false;

        if (fun->isCloneAtCallsite()) {
            RootedScript script(cx);
            jsbytecode *pc;
            types::TypeScript::GetPcScript(cx, script.address(), &pc);
            fun = CloneFunctionAtCallsite(cx, fun0, script, pc);
            if (!fun)
                return false;
        }

        if (!fun->nonLazyScript()->canIonCompile()) {
            UnrootedScript script = GetTopIonJSScript(cx);
            if (script->hasIonScript() &&
                ++script->ion->slowCallCount >= js_IonOptions.slowCallLimit)
            {
                AutoFlushCache afc("InvokeFunction");

                // Poison the script so we don't try to run it again. This will
                // trigger invalidation.
                ForbidCompilation(cx, script);
            }
        }

        // When caller runs in IM, but callee not, we take a slow path to the interpreter.
        // This has a significant overhead. In order to decrease the number of times this happens,
        // the useCount gets incremented faster to compile this function in IM and use the fastpath.
        fun->nonLazyScript()->incUseCount(js_IonOptions.slowCallIncUseCount);
    }

    // TI will return false for monitorReturnTypes, meaning there is no
    // TypeBarrier or Monitor instruction following this. However, we need to
    // explicitly monitor if the callee has not been analyzed yet. We special
    // case this to avoid the cost of ion::GetPcScript if we must take this
    // path frequently.
    bool needsMonitor = ShouldMonitorReturnType(fun);

    // Data in the argument vector is arranged for a JIT -> JIT call.
    Value thisv = argv[0];
    Value *argvWithoutThis = argv + 1;

    // For constructing functions, |this| is constructed at caller side and we can just call Invoke.
    // When creating this failed / is impossible at caller site, i.e. MagicValue(JS_IS_CONSTRUCTING),
    // we use InvokeConstructor that creates it at the callee side.
    bool ok;
    if (thisv.isMagic(JS_IS_CONSTRUCTING))
        ok = InvokeConstructor(cx, ObjectValue(*fun), argc, argvWithoutThis, rval);
    else
        ok = Invoke(cx, thisv, ObjectValue(*fun), argc, argvWithoutThis, rval);

    if (ok && needsMonitor)
        types::TypeScript::Monitor(cx, *rval);

    return ok;
}

JSObject *
NewGCThing(JSContext *cx, gc::AllocKind allocKind, size_t thingSize)
{
    return gc::NewGCThing<JSObject>(cx, allocKind, thingSize);
}

bool
CheckOverRecursed(JSContext *cx)
{
    // IonMonkey's stackLimit is equal to nativeStackLimit by default. When we
    // want to trigger an operation callback, we set the ionStackLimit to NULL,
    // which causes the stack limit check to fail.
    //
    // There are two states we're concerned about here:
    //   (1) The interrupt bit is set, and we need to fire the interrupt callback.
    //   (2) The stack limit has been exceeded, and we need to throw an error.
    //
    // Note that we can reach here if ionStackLimit is MAXADDR, but interrupt
    // has not yet been set to 1. That's okay; it will be set to 1 very shortly,
    // and in the interim we might just fire a few useless calls to
    // CheckOverRecursed.
    JS_CHECK_RECURSION(cx, return false);

    if (cx->runtime->interrupt)
        return InterruptCheck(cx);

    return true;
}

bool
DefVarOrConst(JSContext *cx, HandlePropertyName dn, unsigned attrs, HandleObject scopeChain)
{
    // Given the ScopeChain, extract the VarObj.
    RootedObject obj(cx, scopeChain);
    while (!obj->isVarObj())
        obj = obj->enclosingScope();

    return DefVarOrConstOperation(cx, obj, dn, attrs);
}

bool
InitProp(JSContext *cx, HandleObject obj, HandlePropertyName name, HandleValue value)
{
    // Copy the incoming value. This may be overwritten; the return value is discarded.
    RootedValue rval(cx, value);
    RootedId id(cx, NameToId(name));

    if (name == cx->names().proto)
        return baseops::SetPropertyHelper(cx, obj, obj, id, 0, &rval, false);
    return DefineNativeProperty(cx, obj, id, rval, NULL, NULL, JSPROP_ENUMERATE, 0, 0, 0);
}

template<bool Equal>
bool
LooselyEqual(JSContext *cx, HandleValue lhs, HandleValue rhs, JSBool *res)
{
    bool equal;
    if (!js::LooselyEqual(cx, lhs, rhs, &equal))
        return false;
    *res = (equal == Equal);
    return true;
}

template bool LooselyEqual<true>(JSContext *cx, HandleValue lhs, HandleValue rhs, JSBool *res);
template bool LooselyEqual<false>(JSContext *cx, HandleValue lhs, HandleValue rhs, JSBool *res);

template<bool Equal>
bool
StrictlyEqual(JSContext *cx, HandleValue lhs, HandleValue rhs, JSBool *res)
{
    bool equal;
    if (!js::StrictlyEqual(cx, lhs, rhs, &equal))
        return false;
    *res = (equal == Equal);
    return true;
}

template bool StrictlyEqual<true>(JSContext *cx, HandleValue lhs, HandleValue rhs, JSBool *res);
template bool StrictlyEqual<false>(JSContext *cx, HandleValue lhs, HandleValue rhs, JSBool *res);

bool
LessThan(JSContext *cx, HandleValue lhs, HandleValue rhs, JSBool *res)
{
    bool cond;
    if (!LessThanOperation(cx, lhs, rhs, &cond))
        return false;
    *res = cond;
    return true;
}

bool
LessThanOrEqual(JSContext *cx, HandleValue lhs, HandleValue rhs, JSBool *res)
{
    bool cond;
    if (!LessThanOrEqualOperation(cx, lhs, rhs, &cond))
        return false;
    *res = cond;
    return true;
}

bool
GreaterThan(JSContext *cx, HandleValue lhs, HandleValue rhs, JSBool *res)
{
    bool cond;
    if (!GreaterThanOperation(cx, lhs, rhs, &cond))
        return false;
    *res = cond;
    return true;
}

bool
GreaterThanOrEqual(JSContext *cx, HandleValue lhs, HandleValue rhs, JSBool *res)
{
    bool cond;
    if (!GreaterThanOrEqualOperation(cx, lhs, rhs, &cond))
        return false;
    *res = cond;
    return true;
}

template<bool Equal>
bool
StringsEqual(JSContext *cx, HandleString lhs, HandleString rhs, JSBool *res)
{
    bool equal;
    if (!js::EqualStrings(cx, lhs, rhs, &equal))
        return false;
    *res = (equal == Equal);
    return true;
}

template bool StringsEqual<true>(JSContext *cx, HandleString lhs, HandleString rhs, JSBool *res);
template bool StringsEqual<false>(JSContext *cx, HandleString lhs, HandleString rhs, JSBool *res);

JSBool
ObjectEmulatesUndefined(RawObject obj)
{
    AutoAssertNoGC nogc;
    return EmulatesUndefined(obj);
}

bool
IteratorMore(JSContext *cx, HandleObject obj, JSBool *res)
{
    RootedValue tmp(cx);
    if (!js_IteratorMore(cx, obj, &tmp))
        return false;

    *res = tmp.toBoolean();
    return true;
}

JSObject*
NewInitArray(JSContext *cx, uint32_t count, types::TypeObject *typeArg)
{
    RootedTypeObject type(cx, typeArg);
    RootedObject obj(cx, NewDenseAllocatedArray(cx, count));
    if (!obj)
        return NULL;

    if (!type) {
        if (!JSObject::setSingletonType(cx, obj))
            return NULL;

        types::TypeScript::Monitor(cx, ObjectValue(*obj));
    } else {
        obj->setType(type);
    }

    return obj;
}

JSObject*
NewInitObject(JSContext *cx, HandleObject templateObject)
{
    RootedObject obj(cx, CopyInitializerObject(cx, templateObject));

    if (!obj)
        return NULL;

    if (templateObject->hasSingletonType()) {
        if (!JSObject::setSingletonType(cx, obj))
            return NULL;

        types::TypeScript::Monitor(cx, ObjectValue(*obj));
    } else {
        obj->setType(templateObject->type());
    }

    return obj;
}

bool
ArrayPopDense(JSContext *cx, HandleObject obj, MutableHandleValue rval)
{
    JS_ASSERT(obj->isArray());

    AutoDetectInvalidation adi(cx, rval.address());

    Value argv[] = { UndefinedValue(), ObjectValue(*obj) };
    AutoValueArray ava(cx, argv, 2);
    if (!js::array_pop(cx, 0, argv))
        return false;

    // If the result is |undefined|, the array was probably empty and we
    // have to monitor the return value.
    rval.set(argv[0]);
    if (rval.isUndefined())
        types::TypeScript::Monitor(cx, rval);
    return true;
}

bool
ArrayPushDense(JSContext *cx, HandleObject obj, HandleValue v, uint32_t *length)
{
    JS_ASSERT(obj->isArray());

    Value argv[] = { UndefinedValue(), ObjectValue(*obj), v };
    AutoValueArray ava(cx, argv, 3);
    if (!js::array_push(cx, 1, argv))
        return false;

    *length = argv[0].toInt32();
    return true;
}

bool
ArrayShiftDense(JSContext *cx, HandleObject obj, MutableHandleValue rval)
{
    JS_ASSERT(obj->isArray());

    AutoDetectInvalidation adi(cx, rval.address());

    Value argv[] = { UndefinedValue(), ObjectValue(*obj) };
    AutoValueArray ava(cx, argv, 2);
    if (!js::array_shift(cx, 0, argv))
        return false;

    // If the result is |undefined|, the array was probably empty and we
    // have to monitor the return value.
    rval.set(argv[0]);
    if (rval.isUndefined())
        types::TypeScript::Monitor(cx, rval);
    return true;
}

JSObject *
ArrayConcatDense(JSContext *cx, HandleObject obj1, HandleObject obj2, HandleObject res)
{
    JS_ASSERT(obj1->isArray());
    JS_ASSERT(obj2->isArray());
    JS_ASSERT_IF(res, res->isArray());

    if (res) {
        // Fast path if we managed to allocate an object inline.
        if (!js::array_concat_dense(cx, obj1, obj2, res))
            return NULL;
        return res;
    }

    Value argv[] = { UndefinedValue(), ObjectValue(*obj1), ObjectValue(*obj2) };
    AutoValueArray ava(cx, argv, 3);
    if (!js::array_concat(cx, 1, argv))
        return NULL;
    return &argv[0].toObject();
}

bool
CharCodeAt(JSContext *cx, HandleString str, int32_t index, uint32_t *code)
{
    JS_ASSERT(index >= 0 &&
              static_cast<uint32_t>(index) < str->length());

    const jschar *chars = str->getChars(cx);
    if (!chars)
        return false;

    *code = chars[index];
    return true;
}

JSFlatString *
StringFromCharCode(JSContext *cx, int32_t code)
{
    jschar c = jschar(code);

    if (StaticStrings::hasUnit(c))
        return cx->runtime->staticStrings.getUnit(c);

    return js_NewStringCopyN(cx, &c, 1);

}

bool
SetProperty(JSContext *cx, HandleObject obj, HandlePropertyName name, HandleValue value,
            bool strict, bool isSetName)
{
    RootedValue v(cx, value);
    RootedId id(cx, NameToId(name));

    if (JS_LIKELY(!obj->getOps()->setProperty)) {
        unsigned defineHow = isSetName ? DNP_UNQUALIFIED : 0;
        return baseops::SetPropertyHelper(cx, obj, obj, id, defineHow, &v, strict);
    }

    return JSObject::setGeneric(cx, obj, obj, id, &v, strict);
}

bool
InterruptCheck(JSContext *cx)
{
    gc::MaybeVerifyBarriers(cx);

    return !!js_HandleExecutionInterrupt(cx);
}

HeapSlot *
NewSlots(JSRuntime *rt, unsigned nslots)
{
    JS_STATIC_ASSERT(sizeof(Value) == sizeof(HeapSlot));

    Value *slots = reinterpret_cast<Value *>(rt->malloc_(nslots * sizeof(Value)));
    if (!slots)
        return NULL;

    for (unsigned i = 0; i < nslots; i++)
        slots[i] = UndefinedValue();

    return reinterpret_cast<HeapSlot *>(slots);
}

JSObject *
NewCallObject(JSContext *cx, HandleShape shape, HandleTypeObject type, HeapSlot *slots)
{
    return CallObject::create(cx, shape, type, slots);
}

JSObject *
NewStringObject(JSContext *cx, HandleString str)
{
    return StringObject::create(cx, str);
}

bool
SPSEnter(JSContext *cx, HandleScript script)
{
    return cx->runtime->spsProfiler.enter(cx, script, script->function());
}

bool
SPSExit(JSContext *cx, HandleScript script)
{
    cx->runtime->spsProfiler.exit(cx, script, script->function());
    return true;
}

bool
OperatorIn(JSContext *cx, HandleValue key, HandleObject obj, JSBool *out)
{
    RootedValue dummy(cx); // Disregards atomization changes: no way to propagate.
    RootedId id(cx);
    if (!FetchElementId(cx, obj, key, &id, &dummy))
        return false;

    RootedObject obj2(cx);
    RootedShape prop(cx);
    if (!JSObject::lookupGeneric(cx, obj, id, &obj2, &prop))
        return false;

    *out = !!prop;
    return true;
}

bool
GetIntrinsicValue(JSContext *cx, HandlePropertyName name, MutableHandleValue rval)
{
    return cx->global()->getIntrinsicValue(cx, name, rval);
}

bool
CreateThis(JSContext *cx, HandleObject callee, MutableHandleValue rval)
{
    rval.set(MagicValue(JS_IS_CONSTRUCTING));

    if (callee->isFunction()) {
        JSFunction *fun = callee->toFunction();
        if (fun->isInterpreted())
            rval.set(ObjectValue(*js_CreateThisForFunction(cx, callee, false)));
    }

    return true;
}

bool
DebugPrologue(JSContext *cx, BaselineFrame *frame, JSBool *mustReturn)
{
    *mustReturn = false;

    JSTrapStatus status = ScriptDebugPrologue(cx, frame);
    switch (status) {
      case JSTRAP_CONTINUE:
        return true;

      case JSTRAP_RETURN:
        // The script is going to return immediately, so we have to call the
        // debug epilogue handler as well.
        JS_ASSERT(frame->hasReturnValue());
        *mustReturn = true;
        return ion::DebugEpilogue(cx, frame, true);

      case JSTRAP_THROW:
      case JSTRAP_ERROR:
        return false;

      default:
        JS_NOT_REACHED("Invalid trap status");
    }
}

bool
DebugEpilogue(JSContext *cx, BaselineFrame *frame, JSBool ok)
{
    // If ScriptDebugEpilogue returns |true| we have to return the frame's
    // return value. If it returns |false|, the debugger threw an exception.
    // In both cases we have to pop debug scopes.
    ok = ScriptDebugEpilogue(cx, frame, ok);
    JS_ASSERT_IF(ok, frame->hasReturnValue());
    DebugScopes::onPopCall(frame, cx);
    return ok;
}

bool
HandleDebugTrap(JSContext *cx, BaselineFrame *frame, uint8_t *retAddr, JSBool *mustReturn)
{
    *mustReturn = false;

    RootedScript script(cx, GetTopIonJSScript(cx));
    jsbytecode *pc = script->baseline->icEntryFromReturnAddress(retAddr).pc(script);

    JS_ASSERT(cx->compartment->debugMode());
    JS_ASSERT(script->stepModeEnabled() || script->hasBreakpointsAt(pc));

    Value rval;
    JSTrapStatus status = JSTRAP_CONTINUE;
    JSInterruptHook hook = cx->runtime->debugHooks.interruptHook;

    if (hook || script->stepModeEnabled()) {
        if (hook)
            status = hook(cx, script, pc, &rval, cx->runtime->debugHooks.interruptHookData);
        if (status == JSTRAP_CONTINUE && script->stepModeEnabled())
            status = Debugger::onSingleStep(cx, &rval);
    }

    if (status == JSTRAP_CONTINUE && script->hasBreakpointsAt(pc))
        status = Debugger::onTrap(cx, &rval);

    switch (status) {
      case JSTRAP_CONTINUE:
        break;

      case JSTRAP_ERROR:
        return false;

      case JSTRAP_RETURN:
        *mustReturn = true;
        frame->setReturnValue(rval);
        return ion::DebugEpilogue(cx, frame, true);

      case JSTRAP_THROW:
        cx->setPendingException(rval);
        return false;

      default:
        JS_NOT_REACHED("Invalid trap status");
    }

    return true;
}

bool
OnDebuggerStatement(JSContext *cx, BaselineFrame *frame, jsbytecode *pc, JSBool *mustReturn)
{
    *mustReturn = false;

    RootedScript script(cx, frame->script());
    JSTrapStatus status = JSTRAP_CONTINUE;
    Value rval;

    if (JSDebuggerHandler handler = cx->runtime->debugHooks.debuggerHandler)
        status = handler(cx, script, pc, &rval, cx->runtime->debugHooks.debuggerHandlerData);

    if (status == JSTRAP_CONTINUE)
        status = Debugger::onDebuggerStatement(cx, &rval);

    switch (status) {
      case JSTRAP_ERROR:
        return false;

      case JSTRAP_CONTINUE:
        return true;

      case JSTRAP_RETURN:
        frame->setReturnValue(rval);
        *mustReturn = true;
        return ion::DebugEpilogue(cx, frame, true);

      case JSTRAP_THROW:
        cx->setPendingException(rval);
        return false;

      default:
        JS_NOT_REACHED("Invalid trap status");
    }
}

} // namespace ion
} // namespace js
