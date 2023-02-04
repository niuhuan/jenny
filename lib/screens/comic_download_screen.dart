import 'package:flutter/material.dart';
import 'package:jenny/basic/methods.dart';
import 'package:jenny/screens/components/item_builder.dart';

import 'components/comic_info_card.dart';
import 'components/right_click_pop.dart';

class ComicDownloadScreen extends StatefulWidget {
  final AlbumResponse album;

  const ComicDownloadScreen(this.album, {Key? key}) : super(key: key);

  @override
  State<StatefulWidget> createState() => _ComicDownloadScreenState();
}

class _ComicDownloadScreenState extends State<ComicDownloadScreen> {
  late Future _innerDownloadFuture;
  final List<int> _taskedEps = []; // 已经下载的EP
  final List<int> _selectedEps = []; // 选中的EP

  Future _init() async {
    var task = await methods.downloadById(widget.album.id);
    task?.chapters.map((e) => e.id)?.forEach(_taskedEps.add);
  }

  @override
  void initState() {
    _innerDownloadFuture = _init();
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return rightClickPop(child: buildScreen(context), context: context);
  }

  Widget buildScreen(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text("下载 - ${widget.album.name}"),
      ),
      body: ListView(
        children: [
          ComicInfoCard(albumToSimple(widget.album), link: true),
          ItemBuilder(
            future: _innerDownloadFuture,
            onRefresh: () async {},
            successBuilder: (
              BuildContext context,
              AsyncSnapshot snapshot,
            ) {
              List<Series> series = widget.album.series.isEmpty
                  ? [
                      Series(
                        id: widget.album.id,
                        name: widget.album.name,
                        sort: "1",
                      ),
                    ]
                  : widget.album.series;
              return Column(
                mainAxisSize: MainAxisSize.min,
                children: [
                  _buildButtons(widget.album, series),
                  Wrap(
                    alignment: WrapAlignment.spaceAround,
                    runSpacing: 10,
                    spacing: 10,
                    children: series.map(_buildSeries).toList(),
                  ),
                ],
              );
            },
          ),
        ],
      ),
    );
  }

  Widget _buildButtons(AlbumResponse albumResponse, List<Series> series) {
    var theme = Theme.of(context);
    return Container(
      padding: const EdgeInsets.all(5),
      decoration: BoxDecoration(
        border: Border(
          bottom: BorderSide(
            color: Colors.grey.shade200,
          ),
        ),
      ),
      child: Wrap(
        spacing: 10,
        runSpacing: 10,
        alignment: WrapAlignment.spaceAround,
        children: [
          MaterialButton(
            color: theme.colorScheme.secondary,
            textColor: Colors.white,
            onPressed: () {
              setState(() {
                _selectedEps.clear();
                series.map((e) => e.id).forEach((element) {
                  if (!_taskedEps.contains(element)) {
                    _selectedEps.add(element);
                  }
                });
              });
            },
            child: const Text('全选'),
          ),
          MaterialButton(
            color: theme.colorScheme.secondary,
            textColor: Colors.white,
            onPressed: () async {
              List<DownloadCreateChapter> chapters = [];
              for (var element in series) {
                if (_selectedEps.contains(element.id)) {
                  chapters.add(DownloadCreateChapter(
                    id: element.id,
                    name: element.name,
                    sort: element.sort,
                  ));
                }
              }
              if (chapters.isEmpty) {
                return;
              }
              var carte = DownloadCreate(
                album: DownloadCreateAlbum(
                  id: albumResponse.id,
                  name: albumResponse.name,
                  author: albumResponse.author,
                  tags: albumResponse.tags,
                  works: albumResponse.works,
                  description: albumResponse.description,
                ),
                chapters: chapters,
              );
              await methods.createDownload(carte);
              Navigator.pop(context);
            },
            child: const Text('确定下载'),
          ),
        ],
      ),
    );
  }

  Widget _buildSeries(Series e) {
    return Container(
      padding: const EdgeInsets.all(5),
      child: MaterialButton(
        elevation: Theme.of(context).colorScheme.brightness == Brightness.light
            ? 1
            : 0,
        focusElevation: 0,
        onPressed: () {
          _clickOfEp(e.id);
        },
        color: _colorOfEp(e.id),
        child: Text.rich(TextSpan(children: [
          WidgetSpan(
            alignment: PlaceholderAlignment.middle,
            child: _iconOfEp(e.id),
          ),
          WidgetSpan(
            alignment: PlaceholderAlignment.middle,
            child: Container(width: 10),
          ),
          TextSpan(
            text: e.name == "" ? e.sort : "${e.sort} - ${e.name}",
            style: TextStyle(color: _textColorOfEp(e.id)),
          ),
        ])),
      ),
    );
  }

  void _clickOfEp(int id) {
    if (_taskedEps.contains(id)) {
      return;
    }
    if (_selectedEps.contains(id)) {
      setState(() {
        _selectedEps.remove(id);
      });
    } else {
      setState(() {
        _selectedEps.add(id);
      });
    }
  }

  Color _colorOfEp(int id) {
    if (_taskedEps.contains(id)) {
      return Colors.grey.shade300;
    }
    if (_selectedEps.contains(id)) {
      return Colors.blueGrey.shade300;
    }
    return Theme.of(context).colorScheme.brightness == Brightness.light
        ? Colors.white
        : Theme.of(context).textTheme.bodyText1!.color!.withOpacity(.17);
  }

  Icon _iconOfEp(int id) {
    if (_taskedEps.contains(id)) {
      return const Icon(Icons.download_rounded, color: Colors.black);
    }
    if (_selectedEps.contains(id)) {
      return const Icon(Icons.check_box, color: Colors.black);
    }
    return Theme.of(context).colorScheme.brightness == Brightness.light
        ? const Icon(Icons.check_box_outline_blank, color: Colors.black)
        : const Icon(Icons.check_box_outline_blank, color: Colors.white);
  }

  Color _textColorOfEp(int id) {
    if (_taskedEps.contains(id)) {
      return Colors.black;
    }
    if (_selectedEps.contains(id)) {
      return  Theme.of(context).colorScheme.brightness == Brightness.light
          ? Colors.black
          : Colors.black;
    }
    return Theme.of(context).colorScheme.brightness == Brightness.light
        ? Colors.black
        : Colors.white;
  }
}
