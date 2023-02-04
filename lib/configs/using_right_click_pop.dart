/// 自动全屏

import 'dart:io';

import 'package:flutter/material.dart';

import '../basic/commons.dart';
import '../basic/methods.dart';

const _propertyName = "usingRightClickPop";
late bool _usingRightClickPop;

Future<void> initUsingRightClickPop() async {
  _usingRightClickPop =
      (await methods.loadProperty(_propertyName)) == "true";
}

bool currentUsingRightClickPop() {
  return _usingRightClickPop;
}

Future<void> _chooseUsingRightClickPop(BuildContext context) async {
  String? result = await chooseListDialog<String>(context,
      title: "鼠标右键返回上一页", values: ["是", "否"]);
  if (result != null) {
    var target = result == "是";
    await methods.saveProperty(_propertyName, "$target");
    _usingRightClickPop = target;
  }
}

Widget usingRightClickPopSetting() {
  if (!(Platform.isWindows || Platform.isMacOS || Platform.isLinux)) {
    return Container();
  }
  return StatefulBuilder(
    builder: (BuildContext context, void Function(void Function()) setState) {
      return ListTile(
        title: const Text("鼠标右键返回上一页"),
        subtitle: Text(_usingRightClickPop ? "是" : "否"),
        onTap: () async {
          await _chooseUsingRightClickPop(context);
          setState(() {});
        },
      );
    },
  );
}
