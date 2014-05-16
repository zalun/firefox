/* -*- Mode: C++; tab-width: 8; indent-tabs-mode: nil; c-basic-offset: 4 -*-
 * vim: set ts=4 sw=4 et tw=80:
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef mozilla_jsipc_JavaScriptParent__
#define mozilla_jsipc_JavaScriptParent__

#include "JavaScriptBase.h"
#include "mozilla/jsipc/PJavaScriptParent.h"

namespace mozilla {
namespace jsipc {

class JavaScriptParent : public JavaScriptBase<PJavaScriptParent>
{
  public:
    JavaScriptParent();

    bool init();

    void decref();
    void incref();

    void drop(JSObject *obj);

    mozilla::ipc::IProtocol*
    CloneProtocol(Channel* aChannel, ProtocolCloneContext* aCtx) MOZ_OVERRIDE;

  private:
    JSObject *fromId(JSContext *cx, ObjectId objId);
    bool toId(JSContext *cx, JSObject *obj, ObjectId *idp);

  private:
    uintptr_t refcount_;
};

} // jsipc
} // mozilla

#endif // mozilla_jsipc_JavaScriptWrapper_h__

