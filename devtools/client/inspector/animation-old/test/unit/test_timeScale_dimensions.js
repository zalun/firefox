/* -*- indent-tabs-mode: nil; js-indent-level: 2 -*- */
/* vim: set ts=2 et sw=2 tw=80: */
/* Any copyright is dedicated to the Public Domain.
   https://creativecommons.org/publicdomain/zero/1.0/ */

"use strict";

const {require} = ChromeUtils.import("resource://devtools/shared/Loader.jsm", {});
const {TimeScale} = require("devtools/client/inspector/animation-old/utils");

const TEST_ENDDELAY_X = [{
  desc: "Testing positive-endDelay animations",
  animations: [{
    previousStartTime: 0,
    duration: 500,
    playbackRate: 1,
    iterationCount: 3,
    delay: 500,
    endDelay: 500
  }],
  expectedEndDelayX: 80
}, {
  desc: "Testing negative-endDelay animations",
  animations: [{
    previousStartTime: 0,
    duration: 500,
    playbackRate: 1,
    iterationCount: 9,
    delay: 500,
    endDelay: -500
  }],
  expectedEndDelayX: 90
}];

function run_test() {
  info("Test calculating endDelayX");

  // Be independent of possible prior tests
  TimeScale.reset();

  for (const {desc, animations, expectedEndDelayX} of TEST_ENDDELAY_X) {
    info(`Adding animations: ${desc}`);

    for (const state of animations) {
      TimeScale.addAnimation(state);

      const {endDelayX} = TimeScale.getAnimationDimensions({state});
      equal(endDelayX, expectedEndDelayX);

      TimeScale.reset();
    }
  }
}
