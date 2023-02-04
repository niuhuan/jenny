import 'package:flutter/material.dart';
import 'package:jenny/basic/entities.dart';
import 'package:jenny/configs/pager_column_number.dart';
import 'package:jenny/configs/pager_cover_rate.dart';
import 'package:jenny/configs/pager_view_mode.dart';
import 'package:jenny/screens/comic_info_screen.dart';

import '../../basic/commons.dart';
import 'comic_info_card.dart';
import 'images.dart';

class ComicList extends StatefulWidget {
  final bool inScroll;
  final List<ComicBasic> data;
  final Widget? append;
  final ScrollController? controller;
  final Function? onScroll;

  const ComicList({
    Key? key,
    required this.data,
    this.append,
    this.controller,
    this.inScroll = false,
    this.onScroll,
  }) : super(key: key);

  @override
  State<StatefulWidget> createState() => _ComicListState();
}

class _ComicListState extends State<ComicList> {
  @override
  void initState() {
    currentPagerViewModeEvent.subscribe(_setState);
    pageColumnEvent.subscribe(_setState);
    pagerCoverRateEvent.subscribe(_setState);
    super.initState();
  }

  @override
  void dispose() {
    currentPagerViewModeEvent.unsubscribe(_setState);
    pageColumnEvent.unsubscribe(_setState);
    pagerCoverRateEvent.unsubscribe(_setState);
    super.dispose();
  }

  _setState(_) {
    setState(() {});
  }

  @override
  Widget build(BuildContext context) {
    switch (currentPagerViewMode) {
      case PagerViewMode.cover:
        return _buildCoverMode();
      case PagerViewMode.info:
        return _buildInfoMode();
      case PagerViewMode.titleInCover:
        return _buildTitleInCoverMode();
      case PagerViewMode.titleAndCover:
        return _buildTitleAndCoverMode();
    }
  }

  Widget _buildCoverMode() {
    List<Widget> widgets = [];
    for (var i = 0; i < widget.data.length; i++) {
      widgets.add(GestureDetector(
        onTap: () {
          _pushToComicInfo(widget.data[i]);
        },
        child: Card(
          shape: coverShape,
          clipBehavior: Clip.antiAlias,
          child: LayoutBuilder(
            builder: (BuildContext context, BoxConstraints constraints) {
              switch (currentPagerCoverRate) {
                case PagerCoverRate.rate3x4:
                  return JM3x4Cover(
                    comicId: widget.data[i].id,
                    width: constraints.maxWidth,
                    height: constraints.maxHeight,
                  );
                case PagerCoverRate.rateSquare:
                  return JMSquareCover(
                    comicId: widget.data[i].id,
                    width: constraints.maxWidth,
                    height: constraints.maxHeight,
                  );
              }
            },
          ),
        ),
      ));
    }
    if (widget.append != null) {
      widgets.add(widget.append!);
    }
    late final double childAspectRatio;
    switch (currentPagerCoverRate) {
      case PagerCoverRate.rate3x4:
        childAspectRatio = 3 / 4;
        break;
      case PagerCoverRate.rateSquare:
        childAspectRatio = 1;
        break;
    }
    if (widget.inScroll) {
      var columnWidth = MediaQuery.of(context).size.width / pagerColumnNumber;
      var wrap = Wrap(
        alignment: WrapAlignment.spaceAround,
        crossAxisAlignment: WrapCrossAlignment.center,
        runAlignment: WrapAlignment.spaceBetween,
        children: widgets
            .map((e) => SizedBox(
                  width: columnWidth,
                  height: columnWidth / childAspectRatio,
                  child: e,
                ))
            .toList(),
      );
      return wrap;
    }
    final view = GridView.count(
      childAspectRatio: childAspectRatio,
      crossAxisCount: pagerColumnNumber,
      controller: widget.controller,
      physics: const AlwaysScrollableScrollPhysics(),
      children: widgets,
    );
    return NotificationListener(
      child: view,
      onNotification: (scrollNotification) {
        widget.onScroll?.call();
        return true;
      },
    );
  }

  Widget _buildInfoMode() {
    List<Widget> widgets = [];
    for (var i = 0; i < widget.data.length; i++) {
      widgets.add(GestureDetector(
        onTap: () {
          _pushToComicInfo(widget.data[i]);
        },
        child: ComicInfoCard(widget.data[i]),
      ));
    }
    if (widget.append != null) {
      widgets.add(SizedBox(height: 100, child: widget.append!));
    }
    if (widget.inScroll) {
      return Column(children: widgets);
    }
    final view = ListView(
      controller: widget.controller,
      physics: const AlwaysScrollableScrollPhysics(),
      padding: const EdgeInsets.only(top: 10, bottom: 10),
      children: widgets,
    );
    return NotificationListener(
      child: view,
      onNotification: (scrollNotification) {
        widget.onScroll?.call();
        return true;
      },
    );
  }

  Widget _buildTitleInCoverMode() {
    List<Widget> widgets = [];
    for (var i = 0; i < widget.data.length; i++) {
      widgets.add(GestureDetector(
        onTap: () {
          _pushToComicInfo(widget.data[i]);
        },
        child: Card(
          shape: coverShape,
          clipBehavior: Clip.antiAlias,
          child: LayoutBuilder(
            builder: (BuildContext context, BoxConstraints constraints) {
              late final Widget image;
              switch (currentPagerCoverRate) {
                case PagerCoverRate.rate3x4:
                  image = JM3x4Cover(
                    comicId: widget.data[i].id,
                    width: constraints.maxWidth,
                    height: constraints.maxHeight,
                  );
                  break;
                case PagerCoverRate.rateSquare:
                  image = JMSquareCover(
                    comicId: widget.data[i].id,
                    width: constraints.maxWidth,
                    height: constraints.maxHeight,
                  );
                  break;
              }
              return Stack(
                children: [
                  image,
                  Align(
                    alignment: Alignment.bottomCenter,
                    child: Container(
                      padding: const EdgeInsets.all(3),
                      color: Colors.black.withAlpha(180),
                      width: constraints.maxWidth,
                      child: Text(
                        "${widget.data[i].name}\n",
                        maxLines: 2,
                        textAlign: TextAlign.center,
                        style: const TextStyle(
                          color: Colors.white,
                          height: 1.3,
                        ),
                        strutStyle: const StrutStyle(
                          height: 1.3,
                        ),
                      ),
                    ),
                  ),
                ],
              );
            },
          ),
        ),
      ));
    }
    if (widget.append != null) {
      widgets.add(widget.append!);
    }
    late final double childAspectRatio;
    switch (currentPagerCoverRate) {
      case PagerCoverRate.rate3x4:
        childAspectRatio = 3 / 4;
        break;
      case PagerCoverRate.rateSquare:
        childAspectRatio = 1;
        break;
    }
    if (widget.inScroll) {
      var columnWidth = MediaQuery.of(context).size.width / pagerColumnNumber;
      var wrap = Wrap(
        alignment: WrapAlignment.spaceAround,
        crossAxisAlignment: WrapCrossAlignment.center,
        runAlignment: WrapAlignment.spaceBetween,
        children: widgets
            .map((e) => SizedBox(
                  width: columnWidth,
                  height: columnWidth / childAspectRatio,
                  child: e,
                ))
            .toList(),
      );
      return wrap;
    }
    final view = GridView.count(
      childAspectRatio: childAspectRatio,
      crossAxisCount: pagerColumnNumber,
      controller: widget.controller,
      physics: const AlwaysScrollableScrollPhysics(),
      children: widgets,
    );
    return NotificationListener(
      child: view,
      onNotification: (scrollNotification) {
        widget.onScroll?.call();
        return true;
      },
    );
  }

  Widget _buildTitleAndCoverMode() {
    final mq = MediaQuery.of(context);
    final width = (mq.size.width - 20) / pagerColumnNumber;
    late final double height;
    switch (currentPagerCoverRate) {
      case PagerCoverRate.rate3x4:
        height = width * 4 / 3;
        break;
      case PagerCoverRate.rateSquare:
        height = width;
        break;
    }
    List<Widget> widgets = [];
    for (var i = 0; i < widget.data.length; i++) {
      widgets.add(GestureDetector(
        onTap: () {
          _pushToComicInfo(widget.data[i]);
        },
        child: Column(
          children: [
            SizedBox(
              width: width,
              height: height,
              child: Card(
                shape: coverShape,
                clipBehavior: Clip.antiAlias,
                child: LayoutBuilder(
                  builder: (BuildContext context, BoxConstraints constraints) {
                    late final Widget image;
                    switch (currentPagerCoverRate) {
                      case PagerCoverRate.rate3x4:
                        image = JM3x4Cover(
                          comicId: widget.data[i].id,
                          width: constraints.maxWidth,
                          height: constraints.maxHeight,
                        );
                        break;
                      case PagerCoverRate.rateSquare:
                        image = JMSquareCover(
                          comicId: widget.data[i].id,
                          width: constraints.maxWidth,
                          height: constraints.maxHeight,
                        );
                        break;
                    }
                    return image;
                  },
                ),
              ),
            ),
            Container(
              width: width,
              height: 50,
              padding: const EdgeInsets.only(left: 5, right: 5, bottom: 10),
              child: Text(
                "${widget.data[i].name}\n",
                maxLines: 2,
                textAlign: TextAlign.center,
                style: const TextStyle(
                  height: 1.3,
                ),
                strutStyle: const StrutStyle(
                  height: 1.3,
                ),
              ),
            ),
          ],
        ),
      ));
    }
    if (widget.append != null) {
      widgets.add(widget.append!);
    }
    final wrap = Wrap(
      alignment: WrapAlignment.spaceAround,
      crossAxisAlignment: WrapCrossAlignment.center,
      runAlignment: WrapAlignment.spaceBetween,
      children: widgets,
    );
    if (widget.inScroll) {
      return wrap;
    }
    final view = ListView(
      controller: widget.controller,
      physics: const AlwaysScrollableScrollPhysics(),
      padding: const EdgeInsets.all(10.0),
      children: [wrap],
    );
    return NotificationListener(
      child: view,
      onNotification: (scrollNotification) {
        widget.onScroll?.call();
        return true;
      },
    );
  }

  void _pushToComicInfo(ComicBasic data) {
    Navigator.push(context, MaterialPageRoute(builder: (BuildContext context) {
      return ComicInfoScreen(data.id, data);
    }));
  }
}
