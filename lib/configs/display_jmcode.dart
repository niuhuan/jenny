/// 自动全屏

import 'dart:io';

import 'package:flutter/material.dart';

import '../basic/commons.dart';
import '../basic/methods.dart';

const _propertyName = "displayJmcode";
late bool _displayJmcode;

Future<void> initDisplayJmcode() async {
  _displayJmcode =
      (await methods.loadProperty(_propertyName)) == "true";
}

bool currentDisplayJmcode() {
  return _displayJmcode;
}

Future<void> _chooseDisplayJmcode(BuildContext context) async {
  String? result = await chooseListDialog<String>(context,
      title: "显示漫画代码", values: ["是", "否"]);
  if (result != null) {
    var target = result == "是";
    await methods.saveProperty(_propertyName, "$target");
    _displayJmcode = target;
  }
}

Widget displayJmcodeSetting() {
  return StatefulBuilder(
    builder: (BuildContext context, void Function(void Function()) setState) {
      return ListTile(
        title: const Text("显示漫画代码"),
        subtitle: Text(_displayJmcode ? "是" : "否"),
        onTap: () async {
          await _chooseDisplayJmcode(context);
          setState(() {});
        },
      );
    },
  );
}
