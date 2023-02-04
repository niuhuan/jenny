/// 自动全屏

import 'dart:io';

import 'package:flutter/material.dart';

import '../basic/commons.dart';
import '../basic/methods.dart';

const _propertyName = "volumeKeyControl";
late bool _volumeKeyControl;

Future<void> initVolumeKeyControl() async {
  _volumeKeyControl =
      (await methods.loadProperty(_propertyName)) == "true";
}

bool currentVolumeKeyControl() {
  return _volumeKeyControl;
}

Future<void> _chooseVolumeKeyControl(BuildContext context) async {
  String? result = await chooseListDialog<String>(context,
      title: "鼠标右键返回上一页", values: ["是", "否"]);
  if (result != null) {
    var target = result == "是";
    await methods.saveProperty(_propertyName, "$target");
    _volumeKeyControl = target;
  }
}

Widget volumeKeyControlSetting() {
  if (!(Platform.isAndroid)) {
    return Container();
  }
  return StatefulBuilder(
    builder: (BuildContext context, void Function(void Function()) setState) {
      return ListTile(
        title: const Text("音量键翻页"),
        subtitle: Text(_volumeKeyControl ? "是" : "否"),
        onTap: () async {
          await _chooseVolumeKeyControl(context);
          setState(() {});
        },
      );
    },
  );
}
