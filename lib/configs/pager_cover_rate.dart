import 'package:event/event.dart';
import 'package:flutter/material.dart';
import 'package:jenny/basic/commons.dart';
import 'package:jenny/basic/methods.dart';

enum PagerCoverRate {
  rate3x4,
  rateSquare,
}

const _propertyName = "pager_cover_rate";
late PagerCoverRate _pagerCoverRate;

final pagerCoverRateEvent = Event();
PagerCoverRate get currentPagerCoverRate => _pagerCoverRate;

Future initPagerCoverRate() async {
  _pagerCoverRate = _fromString(await methods.loadProperty(_propertyName));
}

PagerCoverRate _fromString(String valueForm) {
  for (var value in PagerCoverRate.values) {
    if (value.toString() == valueForm) {
      return value;
    }
  }
  return PagerCoverRate.values.first;
}

String pagerCoverRateName(PagerCoverRate type) {
  switch (type) {
    case PagerCoverRate.rate3x4:
      return "3X4";
    case PagerCoverRate.rateSquare:
      return "1X1";
  }
}

Future choosePagerCoverRate(BuildContext context) async {
  final Map<String, PagerCoverRate> map = {};
  for (var element in PagerCoverRate.values) {
    map[pagerCoverRateName(element)] = element;
  }
  final newPagerCoverRate = await chooseMapDialog(
    context,
    title: "请选择封面比例",
    values: map,
  );
  if (newPagerCoverRate != null) {
    await methods.saveProperty(_propertyName, "$newPagerCoverRate");
    _pagerCoverRate = newPagerCoverRate;
    pagerCoverRateEvent.broadcast();
  }
}
