import 'package:event/event.dart';
import 'package:flutter/material.dart';
import 'package:jenny/basic/commons.dart';
import 'package:jenny/basic/methods.dart';

const _propertyKey = "pager_controller_mode";
late PagerControllerMode _value;
final currentPagerControllerModeEvent = Event();

PagerControllerMode get currentPagerControllerMode => _value;

enum PagerControllerMode {
  stream,
  pager,
}

Map<PagerControllerMode, String> _nameMap = {
  PagerControllerMode.stream: "流式",
  PagerControllerMode.pager: "分页器",
};

String get currentPagerControllerModeName => _nameMap[_value]!;

Future choosePagerControllerMode(BuildContext context) async {
  final target = await chooseMapDialog(context,
      title: "请选择分页模式",
      values: _nameMap.map((key, value) => MapEntry(value, key)));
  if (target != null && target != _value) {
    await methods.saveProperty(_propertyKey, "$target");
    _value = target;
    currentPagerControllerModeEvent.broadcast();
  }
}

PagerControllerMode _parse(String string) {
  for (var value in PagerControllerMode.values) {
    if ("$value" == string) {
      return value;
    }
  }
  return PagerControllerMode.stream;
}

Future initPagerControllerMode() async {
  _value = _parse(await methods.loadProperty(_propertyKey));
}
