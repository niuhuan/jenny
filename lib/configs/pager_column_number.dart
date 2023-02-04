import 'package:event/event.dart';
import 'package:flutter/material.dart';
import 'package:jenny/basic/commons.dart';
import 'package:jenny/basic/methods.dart';

const _propertyName = "pager_column_number";
late int _pagerColumnNumber;

int get pagerColumnNumber => _pagerColumnNumber;
final pageColumnEvent = Event();

Future initPagerColumnCount() async {
  String numStr = await methods.loadProperty(_propertyName);
  if (numStr == "") {
    numStr = "4";
  }
  _pagerColumnNumber = int.parse(numStr);
}

Future choosePagerColumnCount(BuildContext context) async {
  final choose = await chooseListDialog(
    context,
    title: "分页每行漫画数",
    values: List<int>.generate(10, (i) => i + 1),
  );
  if (choose != null) {
    await methods.saveProperty(_propertyName, choose.toString());
    _pagerColumnNumber = choose;
    pageColumnEvent.broadcast();
  }
}
