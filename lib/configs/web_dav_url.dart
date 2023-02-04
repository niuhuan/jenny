import 'package:flutter/material.dart';

import '../basic/commons.dart';
import '../basic/methods.dart';

late String _currentWebDavUrl;
const _propertyName = "WebDavUrl";

Future<String?> initWebDavUrl() async {
  _currentWebDavUrl  = await methods.loadProperty(_propertyName);
  if (_currentWebDavUrl == "") {
    _currentWebDavUrl = "http://server/.jenny.history";
  }
  return null;
}

String currentWebDavUrlName() {
  return _currentWebDavUrl == "" ? "未设置" : _currentWebDavUrl;
}

String get currentWebDavUrl => _currentWebDavUrl;

Future<dynamic> inputWebDavUrl(BuildContext context) async {
  String? input = await displayTextInputDialog(
    context,
    src: _currentWebDavUrl,
    title: 'WebDAV文件URL',
    hint: '请输入WebDAV文件URL',
    desc: " ( 例如 http://server/folder/.jenny.history ) ",
  );
  if (input != null) {
    await methods.saveProperty(_propertyName, input);
    _currentWebDavUrl = input;
  }
}

Widget webDavUrlSetting() {
  return StatefulBuilder(
    builder: (BuildContext context, void Function(void Function()) setState) {
      return ListTile(
        title: const Text("WebDAV文件URL"),
        subtitle: Text(currentWebDavUrlName()),
        onTap: () async {
          await inputWebDavUrl(context);
          setState(() {});
        },
      );
    },
  );
}
