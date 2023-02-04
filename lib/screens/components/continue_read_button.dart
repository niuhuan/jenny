import 'package:flutter/material.dart';
import 'package:jenny/basic/entities.dart';
import 'package:jenny/basic/methods.dart';
import 'package:jenny/screens/components/my_flat_button.dart';

// 继续阅读按钮
class ContinueReadButton extends StatefulWidget {
  final Future<ViewLog?> viewFuture;
  final AlbumResponse album;
  final Function(int epOrder, int pictureRank) onChoose;

  const ContinueReadButton({
    Key? key,
    required this.album,
    required this.onChoose,
    required this.viewFuture,
  }) : super(key: key);

  @override
  State<StatefulWidget> createState() => _ContinueReadButtonState();
}

class _ContinueReadButtonState extends State<ContinueReadButton> {
  @override
  Widget build(BuildContext context) {
    return FutureBuilder(
      future: widget.viewFuture,
      builder: (BuildContext context, AsyncSnapshot<ViewLog?> snapshot) {
        late void Function() onPressed;
        late String text;
        if (snapshot.connectionState != ConnectionState.done) {
          onPressed = () {};
          text = '加载中';
        } else {
          ViewLog? viewLog = snapshot.data;
          if (viewLog == null || viewLog.lastViewChapterId == 0) {
            if (widget.album.series.isEmpty) {
              return Container();
            }
            onPressed = () {
              if (widget.album.series.isEmpty) {
                widget.onChoose(widget.album.id, 0);
              } else {
                widget.album.series.sort(
                  (a, b) => int.parse(a.sort).compareTo(int.parse(b.sort)),
                );
                widget.onChoose(widget.album.series[0].id, 0);
              }
            };
            text = '从头开始';
          } else {
            onPressed = () {
              widget.onChoose(
                viewLog.lastViewChapterId,
                viewLog.lastViewPage,
              );
            };
            text = '继续阅读'; // todo names and pages
          }
        }
        return MyFlatButton(title: text, onPressed: onPressed);
      },
    );
  }
}
