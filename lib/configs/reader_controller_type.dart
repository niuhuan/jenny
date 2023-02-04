/// 全屏操作

import 'package:flutter/material.dart';
import 'package:jenny/basic/commons.dart';
import 'package:jenny/basic/methods.dart';

enum ReaderControllerType {
  touchOnce,
  controller,
  touchDouble,
  touchDoubleOnceNext,
  threeArea,
}

Map<String, ReaderControllerType> _readerControllerTypeMap = {
  "点击屏幕一次全屏": ReaderControllerType.touchOnce,
  "使用控制器全屏": ReaderControllerType.controller,
  "双击屏幕全屏": ReaderControllerType.touchDouble,
  "双击屏幕全屏 + 单击屏幕下一页": ReaderControllerType.touchDoubleOnceNext,
  "将屏幕划分成三个区域 (上一页, 下一页, 全屏)": ReaderControllerType.threeArea,
};

const _defaultController = ReaderControllerType.touchOnce;
const _propertyName = "reader_controller_type";
late ReaderControllerType _readerControllerType;

Future<void> initReaderControllerType() async {
  _readerControllerType =
      _readerControllerTypeFromString(await methods.loadProperty(
    _propertyName,
  ));
}

ReaderControllerType get currentReaderControllerType => _readerControllerType;

ReaderControllerType _readerControllerTypeFromString(String string) {
  for (var value in ReaderControllerType.values) {
    if (string == value.toString()) {
      return value;
    }
  }
  return _defaultController;
}

String currentReaderControllerTypeName() {
  for (var e in _readerControllerTypeMap.entries) {
    if (e.value == _readerControllerType) {
      return e.key;
    }
  }
  return '';
}

Future<void> chooseReaderControllerType(BuildContext context) async {
  ReaderControllerType? result = await chooseMapDialog<ReaderControllerType>(
    context,
    title: "选择操控方式",
    values: _readerControllerTypeMap,
  );
  if (result != null) {
    await methods.saveProperty(_propertyName, result.toString());
    _readerControllerType = result;
  }
}
