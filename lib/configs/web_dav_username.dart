
import 'package:flutter/material.dart';

import '../basic/commons.dart';
import '../basic/methods.dart';

late String _currentWebDavUserName;
const _propertyName = "WebDavUserName";

String get currentWebUserName => _currentWebDavUserName;

Future<String?> initWebDavUserName() async {
  _currentWebDavUserName  = await methods.loadProperty(_propertyName);
  return null;
}

String currentWebDavUserNameName() {
  return _currentWebDavUserName == "" ? "未设置" : _currentWebDavUserName;
}

Future<dynamic> inputWebDavUserName(BuildContext context) async {
  String? input = await displayTextInputDialog(
    context,
    src: _currentWebDavUserName,
    title: 'WebDAV用户名',
    hint: '请输入WebDAV用户名',
  );
  if (input != null) {
    await methods.saveProperty(_propertyName, input);
    _currentWebDavUserName = input;
  }
}

Widget webDavUserNameSetting() {
  return StatefulBuilder(
    builder: (BuildContext context, void Function(void Function()) setState) {
      return ListTile(
        title: const Text("WebDAV用户名"),
        subtitle: Text(currentWebDavUserNameName()),
        onTap: () async {
          await inputWebDavUserName(context);
          setState(() {});
        },
      );
    },
  );
}
