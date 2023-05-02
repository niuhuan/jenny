/// 自动全屏

import 'dart:io';

import 'package:flutter/material.dart';

import '../basic/commons.dart';
import '../basic/methods.dart';

const _propertyName = "passed";
late bool _passed;

Future<void> initPassed() async {
  _passed = (await methods.loadProperty(_propertyName)) == "true";
}

bool currentPassed() {
  return _passed;
}

Future<void> firstPassed() async {
  await methods.saveProperty(_propertyName, "true");
}
