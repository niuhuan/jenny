import 'dart:io';

import 'package:file_picker/file_picker.dart';
import 'package:filesystem_picker/filesystem_picker.dart';
import 'package:flutter/material.dart';
import 'package:permission_handler/permission_handler.dart';

import '../basic/commons.dart';
import '../basic/methods.dart';
import '../configs/import_notice.dart';
import '../configs/is_pro.dart';
import '../configs/android_version.dart';
import 'components/content_loading.dart';
import 'components/right_click_pop.dart';

// 导入
class DownloadImportScreen extends StatefulWidget {
  const DownloadImportScreen({Key? key}) : super(key: key);

  @override
  State<StatefulWidget> createState() => _DownloadImportScreenState();
}

class _DownloadImportScreenState extends State<DownloadImportScreen> {
  bool _importing = false;
  String _importMessage = "";

  @override
  void initState() {
    // registerEvent(_onMessageChange, "EXPORT");
    super.initState();
  }

  @override
  void dispose() {
    // unregisterEvent(_onMessageChange);
    super.dispose();
  }

  void _onMessageChange(event) {
    if (event is String) {
      setState(() {
        _importMessage = event;
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    return rightClickPop(
      child: buildScreen(context),
      context: context,
      canPop: !_importing,
    );
  }

  Widget buildScreen(BuildContext context) {
    if (_importing) {
      return Scaffold(
        body: ContentLoading(label: _importMessage),
      );
    }

    return Scaffold(
      appBar: AppBar(
        title: const Text('导入'),
      ),
      body: ListView(
        children: [
          Container(
            padding: const EdgeInsets.all(10),
            child: Text(_importMessage),
          ),
          Container(height: 20),
          importNotice(context),
          Container(height: 20),
          _fileImportButton(),
          Container(height: 20),
          _importDirFilesZipButton(),
          Container(height: 20),
          Container(height: 20),
        ],
      ),
    );
  }

  Widget _fileImportButton() {
    return MaterialButton(
      height: 80,
      onPressed: () async {
        if(!await androidMangeStorageRequest()) {
          defaultToast(context, "申请权限被拒绝");
        }
        String? path;
        if (Platform.isAndroid) {
          path = await FilesystemPicker.open(
            title: 'Open file',
            context: context,
            rootDirectory: Directory("/storage/emulated/0"),
            fsType: FilesystemType.file,
            folderIconColor: Colors.teal,
            allowedExtensions: ['.zip', '.jmi'],
            fileTileSelectMode: FileTileSelectMode.wholeTile,
          );
        } else {
          var ls = await FilePicker.platform.pickFiles(
            dialogTitle: '选择要导入的文件',
            allowMultiple: false,
            type: FileType.custom,
            allowedExtensions: ['zip', 'jmi'],
            allowCompression: false,
          );
          path = ls != null && ls.count > 0 ? ls.paths[0] : null;
        }
        if (path != null) {
          if (path.endsWith(".jm.zip") || path.endsWith(".jmi")) {
            try {
              setState(() {
                _importing = true;
              });
              if (path.endsWith(".zip")) {
                await methods.import_jm_zip(path);
              } else if (path.endsWith(".jmi")) {
                await methods.import_jm_jmi(path);
              }
              setState(() {
                _importMessage = "导入成功";
              });
            } catch (e) {
              setState(() {
                _importMessage = "导入失败 $e";
              });
            } finally {
              setState(() {
                _importing = false;
              });
            }
          } else if (path.endsWith(".jm.zip")) {
            defaultToast(context, "只能导入.jm.zip的zip压缩包");
          }
        }
      },
      child: Text(
        '选择.jm.zip文件进行导入\n选择jmi文件进行导入' + (!isPro ? "\n(发电后使用)" : ""),
        style: TextStyle(
          color: !isPro ? Colors.grey : null,
        ),
        textAlign: TextAlign.center,
      ),
    );
  }

  Widget _importDirFilesZipButton() {
    return MaterialButton(
      height: 80,
      onPressed: () async {
        if(!await androidMangeStorageRequest()) {
          throw Exception("申请权限被拒绝");
        }
        late String? path;
        try {
          path = await chooseFolder(context);
        } catch (e) {
          defaultToast(context, "$e");
          return;
        }
        if (path != null) {
          try {
            setState(() {
              _importing = true;
            });
            await methods.import_jm_dir(path);
            setState(() {
              _importMessage = "导入成功";
            });
          } catch (e) {
            setState(() {
              _importMessage = "导入失败 $e";
            });
          } finally {
            setState(() {
              _importing = false;
            });
          }
        }
      },
      child: Text(
        '选择文件夹\n(导入里面所有的zip/jmi)' + (!isPro ? "\n(发电后使用)" : ""),
        style: TextStyle(
          color: !isPro ? Colors.grey : null,
        ),
        textAlign: TextAlign.center,
      ),
    );
  }
}
