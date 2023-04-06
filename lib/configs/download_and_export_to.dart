import 'dart:io';

import 'package:flutter/material.dart';
import 'package:permission_handler/permission_handler.dart';

import '../basic/commons.dart';
import '../basic/methods.dart';
import 'android_version.dart';
import 'is_pro.dart';

late String _currentDownloadAndExportTo;
const _propertyName = "DownloadAndExportTo";

Future<String?> initDownloadAndExportTo() async {
  _currentDownloadAndExportTo = await methods.getDownloadAndExportTo();
  return null;
}

String currentDownloadAndExportToName() {
  return _currentDownloadAndExportTo == ""
      ? "未设置"
      : _currentDownloadAndExportTo;
}

String get currentDownloadAndExportTo => _currentDownloadAndExportTo;

Widget downloadAndExportToSetting() {
  if (!isPro) {
    return SwitchListTile(
      title: const Text("下载时同时导出", style: TextStyle(color: Colors.grey)),
      subtitle: const Text("发电才能使用", style: TextStyle(color: Colors.grey)),
      value: false,
      onChanged: (_) {},
    );
  }
  if (Platform.isIOS) {
    return StatefulBuilder(
      builder: (BuildContext context, void Function(void Function()) setState) {
        return SwitchListTile(
          title: const Text("下载时同时导出"),
          subtitle: Text(_currentDownloadAndExportTo),
          value: _currentDownloadAndExportTo.isNotEmpty,
          onChanged: (e) async {

            var root =
                e ? ((await methods.iosGetDocumentDir()) + "/exports") : "";
            await methods.setDownloadAndExportTo(root);
            _currentDownloadAndExportTo = root;
            setState(() {});
          },
        );
      },
    );
  }
  return StatefulBuilder(
    builder: (BuildContext context, void Function(void Function()) setState) {
      return ListTile(
        title: const Text("下载的同时导出到某个目录(填完整路径)"),
        subtitle: Text(currentDownloadAndExportToName()),
        onTap: () async {
          var result = await chooseListDialog(context,
              values: ["选择新位置", "清除设置"], title: "下载的时候同时导出");
          if (result != null) {
            if ("选择新位置" == result) {
              if(!await androidMangeStorageRequest()) {
                throw Exception("申请权限被拒绝");
              }
              String? root = await chooseFolder(context);
              if (root != null) {
                await methods.setDownloadAndExportTo(root);
                _currentDownloadAndExportTo = root;
                setState(() {});
              }
            } else if ("清除设置" == result) {
              const root = "";
              await methods.setDownloadAndExportTo(root);
              _currentDownloadAndExportTo = root;
              setState(() {});
            }
          }
        },
      );
    },
  );
}
