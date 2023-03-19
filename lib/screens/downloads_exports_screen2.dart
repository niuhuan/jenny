import 'dart:io';

import 'package:flutter/material.dart';
import 'package:jenny/basic/methods.dart';
import 'package:jenny/screens/components/content_builder.dart';
import 'package:jenny/screens/downloads_exporting_screen2.dart';
import 'package:permission_handler/permission_handler.dart';
import '../basic/commons.dart';
import 'components/comic_download_card.dart';
import 'components/right_click_pop.dart';
import '../configs/android_version.dart';

class DownloadsExportScreen2 extends StatefulWidget {
  const DownloadsExportScreen2({Key? key}) : super(key: key);

  @override
  State<StatefulWidget> createState() => _DownloadsExportScreen2State();
}

class _DownloadsExportScreen2State extends State<DownloadsExportScreen2> {
  late Future<List<DownloadAlbum>> _downloadsFuture;

  @override
  void initState() {
    _downloadsFuture = methods.allDownloads().then((value) {
      List<DownloadAlbum> a = [];
      for (var value1 in value) {
        a.add(value1);
      }
      return a;
    });
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return rightClickPop(child: buildScreen(context), context: context);
  }

  Widget buildScreen(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text("批量导出(即使没有下载完)"),
        actions: [
          FutureBuilder(
            future: _downloadsFuture,
            builder: (BuildContext context,
                AsyncSnapshot<List<DownloadAlbum>> snapshot) {
              if (snapshot.connectionState != ConnectionState.done) {
                return Container();
              }
              List<int> exportableIds = [];
              for (var value in snapshot.requireData) {
                exportableIds.add(value.id);
              }
              return _selectAllButton(exportableIds);
            },
          ),
          _goToExport(),
        ],
      ),
      body: ContentBuilder(
        key: null,
        future: _downloadsFuture,
        onRefresh: () async {
          setState(() {
            _downloadsFuture = methods.allDownloads();
          });
        },
        successBuilder: (
          BuildContext context,
          AsyncSnapshot<List<DownloadAlbum>> snapshot,
        ) {
          return ListView(
            children: snapshot.requireData
                .map((e) => GestureDetector(
                      onTap: () {
                        if (selected.contains(e.id)) {
                          selected.remove(e.id);
                        } else {
                          selected.add(e.id);
                        }
                        setState(() {});
                      },
                      child: Stack(children: [
                        ComicDownloadCard(e),
                        Row(children: [
                          Expanded(child: Container()),
                          Padding(
                            padding: const EdgeInsets.all(5),
                            child: Icon(
                              selected.contains(e.id)
                                  ? Icons.check_circle_sharp
                                  : Icons.circle_outlined,
                              color: Theme.of(context).colorScheme.secondary,
                            ),
                          ),
                        ]),
                      ]),
                    ))
                .toList(),
          );
        },
      ),
    );
  }

  List<int> selected = [];

  Widget _selectAllButton(List<int> exportableIds) {
    return MaterialButton(
        minWidth: 0,
        onPressed: () async {
          setState(() {
            if (selected.length >= exportableIds.length) {
              selected.clear();
            } else {
              selected.clear();
              selected.addAll(exportableIds);
            }
          });
        },
        child: Column(
          children: [
            Expanded(child: Container()),
            const Icon(
              Icons.select_all,
              size: 18,
              color: Colors.white,
            ),
            const Text(
              '全选',
              style: TextStyle(fontSize: 14, color: Colors.white),
            ),
            Expanded(child: Container()),
          ],
        ));
  }

  Widget _goToExport() {
    return MaterialButton(
        minWidth: 0,
        onPressed: () async {
          if (selected.isEmpty) {
            defaultToast(context, "请选择导出的内容");
            return;
          }
          if (Platform.isAndroid) {
            if (androidVersion >= 30) {
              if (!(await Permission.manageExternalStorage.request())
                  .isGranted) {
                throw Exception("申请权限被拒绝");
              }
            } else {
              if (!(await Permission.storage.request()).isGranted) {
                throw Exception("申请权限被拒绝");
              }
            }
          }
          final exported = await Navigator.of(context).push(
            MaterialPageRoute(
              builder: (context) => DownloadsExportingScreen2(
                idList: selected,
              ),
            ),
          );
          _downloadsFuture = methods.allDownloads().then((value) {
            List<DownloadAlbum> a = [];
            for (var value1 in value) {
              a.add(value1);
            }
            return a;
          });
          var pre = selected;
          setState(() {
            selected = [];
          });
          final result = await _downloadsFuture;
          for (var value2 in result.map((e) => e.id)) {
            if (pre.contains(value2)) {
              selected.add(value2);
            }
          }
          setState(() {});
        },
        child: Column(
          children: [
            Expanded(child: Container()),
            const Icon(
              Icons.check,
              size: 18,
              color: Colors.white,
            ),
            const Text(
              '确认',
              style: TextStyle(fontSize: 14, color: Colors.white),
            ),
            Expanded(child: Container()),
          ],
        ));
  }
}
