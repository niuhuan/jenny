import 'package:flutter/material.dart';
import 'package:jenny/basic/commons.dart';
import 'package:jenny/basic/methods.dart';
import 'package:jenny/configs/download_thread_count.dart';
import 'package:jenny/screens/components/content_builder.dart';
import 'package:jenny/screens/components/content_loading.dart';
import 'package:jenny/screens/download_import_screen.dart';

import 'components/comic_download_card.dart';
import 'components/right_click_pop.dart';
import 'download_album_screen.dart';
import 'downloads_exports_screen.dart';

class DownloadsScreen extends StatefulWidget {
  const DownloadsScreen({Key? key}) : super(key: key);

  @override
  State<StatefulWidget> createState() => _DownloadsScreenState();
}

class _DownloadsScreenState extends State<DownloadsScreen> {
  bool _loading = true;
  List<DownloadAlbum> _downloads = [];

  _load() async {
    setState(() {
      _loading = true;
    });
    try {
      _downloads = await methods.allDownloads();
    } catch (_e) {
      // 极端情况才发生, 忽略
    } finally {
      _loading = false;
    }
  }

  @override
  void initState() {
    _load();
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return rightClickPop(child: buildScreen(context), context: context);
  }

  Widget buildScreen(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text("下载"),
        actions: [
          threadCountButton(),
          exportButton(),
          importButton(),
          IconButton(
            onPressed: () async {
              await methods.renewAllDownloads();
              if (!_loading) {
                _load();
              }
            },
            icon: const Icon(Icons.autorenew),
          ),
        ],
      ),
      body: _body(),
    );
  }

  Widget _body(){
    if (_loading && _downloads.isEmpty) {
      return const ContentLoading(label: "加载中");
    }
    // if (_loading) 可以加个浮层, 极为短暂, 性价比比较低
    return _listView();
  }

  Widget _listView(){
     return ListView(
      children: _downloads.map((e) => GestureDetector(
        key: Key("DOWNLOAD:${e.id}"),
        onTap: () {
          if (e.dlStatus == 3) {
            return;
          }
          Navigator.of(context).push(
            MaterialPageRoute(builder: (BuildContext context) {
              return DownloadAlbumScreen(e);
            }),
          );
        },
        onLongPress: () async {
          String? action = await chooseListDialog(context,
              values: ["删除"], title: "请选择");
          if (action != null && action == "删除") {
            await methods.deleteDownload(e.id);
            _load();
          }
        },
        child: ComicDownloadCard(e),
      ))
          .toList(),
    );
  }

  Widget importButton() {
    return IconButton(
      onPressed: () async {
        await Navigator.push(
          context,
          MaterialPageRoute(
            builder: (context) => const DownloadImportScreen(),
          ),
        );
        _load();
      },
      icon: const Icon(
        Icons.drive_folder_upload,
      ),
    );
  }

  Widget exportButton() {
    return IconButton(
      onPressed: () async {
        await Navigator.push(
          context,
          MaterialPageRoute(
            builder: (context) => const DownloadsExportScreen(),
          ),
        );
      },
      icon: const Icon(
        Icons.sim_card_download_outlined,
      ),
    );
  }

  Widget threadCountButton() {
    return MaterialButton(
      onPressed: () async {
        await chooseDownloadThread(context);
        setState(() {});
      },
      minWidth: 0,
      child: Text(
        "$downloadThreadCount线程",
      ),
    );
  }
}
