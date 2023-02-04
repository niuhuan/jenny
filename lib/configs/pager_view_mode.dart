import 'package:event/event.dart';
import 'package:flutter/material.dart';
import 'package:jenny/basic/commons.dart';
import 'package:jenny/basic/methods.dart';

const _propertyKey = "pager_view_mode";
late PagerViewMode _value;
final Event currentPagerViewModeEvent = Event();

PagerViewMode get currentPagerViewMode => _value;

enum PagerViewMode {
  cover,
  info,
  titleInCover,
  titleAndCover,
}

Map<PagerViewMode, String> _nameMap = {
  PagerViewMode.cover: "封面",
  PagerViewMode.info: "详情",
  PagerViewMode.titleInCover: "图文1",
  PagerViewMode.titleAndCover: "图文2",
};

String get currentPagerViewModeName => _nameMap[_value]!;

Future choosePagerViewMode(BuildContext context) async {
  final target = await chooseMapDialog(context,
      title: "请选择展现形式",
      values: _nameMap.map((key, value) => MapEntry(value, key)));
  if (target != null && target != _value) {
    await methods.saveProperty(_propertyKey, "$target");
    _value = target;
    currentPagerViewModeEvent.broadcast();
  }
}

PagerViewMode _parse(String string) {
  for (var value in PagerViewMode.values) {
    if ("$value" == string) {
      return value;
    }
  }
  return PagerViewMode.cover;
}

Future initPagerViewMode() async {
  _value = _parse(await methods.loadProperty(_propertyKey));
}
