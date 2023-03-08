import 'dart:io';

import 'package:flutter/material.dart';
import 'package:jenny/basic/methods.dart';
import 'package:jenny/screens/components/content_builder.dart';
import 'package:permission_handler/permission_handler.dart';
import '../basic/commons.dart';
import 'components/comic_download_card.dart';
import 'components/right_click_pop.dart';
import 'downloads_exporting_screen.dart';
import '../configs/android_version.dart';

class DownloadsExportScreen extends StatefulWidget {
  const DownloadsExportScreen({Key? key}) : super(key: key);

  @override
  State<StatefulWidget> createState() => _DownloadsExportScreenState();
}

class _DownloadsExportScreenState extends State<DownloadsExportScreen> {
  late Future<List<DownloadAlbum>> _downloadsFuture;

  @override
  void initState() {
    _downloadsFuture = methods.allDownloads().then((value) {
      List<DownloadAlbum> a = [];
      for (var value1 in value) {
        if (value1.dlStatus == 1) {
          a.add(value1);
        }
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
        title: const Text("批量导出"),
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
    return IconButton(
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
      icon: const Icon(
        Icons.select_all,
      ),
    );
  }

  Widget _goToExport() {
    return IconButton(
      onPressed: () async {
        if (selected.isEmpty) {
          defaultToast(context, "请选择导出的内容");
          return;
        }
        if (Platform.isAndroid) {
          if (androidVersion >= 30) {
            if (!(await Permission.manageExternalStorage.request()).isGranted) {
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
            builder: (context) => DownloadsExportingScreen(
              idList: selected,
            ),
          ),
        );
      },
      icon: const Icon(
        Icons.check,
      ),
    );
  }
}
