/// 代理设置

import 'dart:io';

import 'package:flutter/material.dart';
import 'package:permission_handler/permission_handler.dart';

import '../basic/commons.dart';
import '../basic/methods.dart';

const String _propertyKey = "export_path";
late String _currentExportPath;

Future<String?> initExportPath() async {
  _currentExportPath = await methods.loadProperty(_propertyKey);
  if (_currentExportPath.isEmpty) {
    if (Platform.isAndroid) {
      try {
        _currentExportPath = await methods.androidDefaultExportsDir();
      } catch (e) {
        _currentExportPath = "/sdcard/Download/jenny/exports";
      }
    } else if (Platform.isMacOS || Platform.isLinux) {
      _currentExportPath = await methods.getHomeDir();
      if (Platform.isMacOS) {
        _currentExportPath = _currentExportPath + "/Downloads";
      }
    } else if (Platform.isWindows) {
      _currentExportPath = "exports";
    }
  }
  return null;
}

String showExportPath() {
  if (Platform.isIOS) {
    return "\n\n随后可在文件管理中找到导出的内容";
  }
  return "\n\n$_currentExportPath";
}

Future _setExportPath(String folder) async {
  await methods.saveProperty(_propertyKey, folder);
  _currentExportPath = folder;
}

Widget displayExportPathInfo() {
  return StatefulBuilder(
    builder: (BuildContext context, void Function(void Function()) setState) {
      if (Platform.isIOS) {
        return Container(
          margin: const EdgeInsets.all(15),
          padding: const EdgeInsets.all(15),
          color: (Theme.of(context).textTheme.bodyText1?.color ?? Colors.black)
              .withOpacity(.01),
          child: const Text("您正在使用iOS设备:\n导出到文件的内容请打开系统自带文件管理进行浏览"),
        );
      }
      return Column(children: [
        ListTile(
          onTap: () async {
            String? choose = await chooseFolder(context);
            if (choose != null) {
              _setExportPath(choose);
            }
            setState(() {});
          },
          title: const Text("导出路径 (点击可修改)"),
          subtitle: Text(_currentExportPath),
        ),
        ...Platform.isAndroid
            ? [
                Container(height: 15),
                Container(
                  margin: const EdgeInsets.all(15),
                  padding: const EdgeInsets.all(15),
                  color: (Theme.of(context).textTheme.bodyText1?.color ??
                          Colors.black)
                      .withOpacity(.01),
                  child: const Text(
                    "您正在使用安卓设备:\n如果不能成功导出并且提示权限不足, 可以尝试在Download或Document下建立子目录进行导出",
                  ),
                ),
              ]
            : [],
      ]);
    },
  );
}

Future<String> attachExportPath() async {
  late String path;
  if (Platform.isIOS) {
    path = await methods.iosGetDocumentDir();
  } else {
    if (Platform.isAndroid) {
      if (!(await Permission.storage.request()).isGranted) {
        throw Exception("申请权限被拒绝");
      }
    }
    path = _currentExportPath;
  }
  if (Platform.isMacOS || Platform.isWindows || Platform.isLinux) {
    await methods.mkdirs(path);
  } else if (Platform.isAndroid) {
    await methods.androidMkdirs(path);
  }
  return path;
}
