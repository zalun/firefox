// Load in the test harness
var scriptLoader = Components.classes["@mozilla.org/moz/jssubscript-loader;1"]
                             .getService(Components.interfaces.mozIJSSubScriptLoader);
scriptLoader.loadSubScript("chrome://mochikit/content/browser/xpinstall/tests/harness.js", this);

// ----------------------------------------------------------------------------
// Test that an install that requires cookies to be sent succeeds when cookies
// are set and third party cookies are disabled.
// This verifies bug 462739
function test() {
  Harness.installEndedCallback = check_xpi_install;
  Harness.installsCompletedCallback = finish_test;
  Harness.setup();

  var cm = Components.classes["@mozilla.org/cookiemanager;1"]
                     .getService(Components.interfaces.nsICookieManager2);
  cm.add("example.com", "/browser/xpinstall/tests", "xpinstall", "true", false,
         false, true, (Date.now() / 1000) + 60);

  var pm = Components.classes["@mozilla.org/permissionmanager;1"]
                     .getService(Components.interfaces.nsIPermissionManager);
  pm.add(makeURI("http://example.com/"), "install", pm.ALLOW_ACTION);

  var prefs = Components.classes["@mozilla.org/preferences-service;1"]
                        .getService(Components.interfaces.nsIPrefBranch);
  prefs.setIntPref("network.cookie.cookieBehavior", 1);

  var triggers = encodeURIComponent(JSON.stringify({
    "Cookie check": TESTROOT + "cookieRedirect.sjs?" + TESTROOT + "unsigned.xpi"
  }));
  gBrowser.selectedTab = gBrowser.addTab();
  gBrowser.loadURI(TESTROOT + "installtrigger.html?" + triggers);
}

function check_xpi_install(addon, status) {
  is(status, 0, "Install should succeed");
}

function finish_test() {
  var em = Components.classes["@mozilla.org/extensions/manager;1"]
                     .getService(Components.interfaces.nsIExtensionManager);
  em.cancelInstallItem("unsigned-xpi@tests.mozilla.org");

  var cm = Components.classes["@mozilla.org/cookiemanager;1"]
                     .getService(Components.interfaces.nsICookieManager2);
  cm.remove("example.com", "xpinstall", "/browser/xpinstall/tests", false);

  var prefs = Components.classes["@mozilla.org/preferences-service;1"]
                        .getService(Components.interfaces.nsIPrefBranch);
  prefs.clearUserPref("network.cookie.cookieBehavior");

  var pm = Components.classes["@mozilla.org/permissionmanager;1"]
                     .getService(Components.interfaces.nsIPermissionManager);
  pm.remove("example.com", "install");

  gBrowser.removeCurrentTab();
  Harness.finish();
}
// ----------------------------------------------------------------------------
