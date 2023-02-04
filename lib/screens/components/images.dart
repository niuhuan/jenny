import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:flutter_svg/flutter_svg.dart';
import 'package:flutter_svg/svg.dart';
import 'package:jenny/basic/commons.dart';
import 'dart:io';
import 'dart:ui' as ui show Codec;

import 'package:jenny/basic/methods.dart';

import '../file_photo_view_screen.dart';

//JM3x4Cover
class JM3x4ImageProvider extends ImageProvider<JM3x4ImageProvider> {
  final int comicId;
  final double scale;

  JM3x4ImageProvider(this.comicId, {this.scale = 1.0});

  @override
  ImageStreamCompleter load(JM3x4ImageProvider key, DecoderCallback decode) {
    return MultiFrameImageStreamCompleter(
      codec: _loadAsync(key),
      scale: key.scale,
    );
  }

  @override
  Future<JM3x4ImageProvider> obtainKey(ImageConfiguration configuration) {
    return SynchronousFuture<JM3x4ImageProvider>(this);
  }

  Future<ui.Codec> _loadAsync(JM3x4ImageProvider key) async {
    assert(key == this);
    return PaintingBinding.instance!.instantiateImageCodec(
      await File(await methods.jm3x4Cover(comicId)).readAsBytes(),
    );
  }

  @override
  bool operator ==(dynamic other) {
    if (other.runtimeType != runtimeType) return false;
    final JM3x4ImageProvider typedOther = other;
    return comicId == typedOther.comicId && scale == typedOther.scale;
  }

  @override
  int get hashCode => hashValues(comicId, scale);

  @override
  String toString() => '$runtimeType('
      ' comicId: ${describeIdentity(comicId)},'
      ' scale: $scale'
      ')';
}

//JM3x4Cover
class PageImageProvider extends ImageProvider<PageImageProvider> {
  final int id;
  final String imageName;
  final double scale;

  PageImageProvider(this.id, this.imageName, {this.scale = 1.0});

  @override
  ImageStreamCompleter load(PageImageProvider key, DecoderCallback decode) {
    return MultiFrameImageStreamCompleter(
      codec: _loadAsync(key),
      scale: key.scale,
    );
  }

  @override
  Future<PageImageProvider> obtainKey(ImageConfiguration configuration) {
    return SynchronousFuture<PageImageProvider>(this);
  }

  Future<ui.Codec> _loadAsync(PageImageProvider key) async {
    assert(key == this);
    return PaintingBinding.instance!.instantiateImageCodec(
      await File(await methods.jmPageImage(id, imageName)).readAsBytes(),
    );
  }

  @override
  bool operator ==(dynamic other) {
    if (other.runtimeType != runtimeType) return false;
    final PageImageProvider typedOther = other;
    return id == typedOther.id &&
        imageName == typedOther.imageName &&
        scale == typedOther.scale;
  }

  @override
  int get hashCode => hashValues(id, imageName, scale);

  @override
  String toString() => '$runtimeType('
      ' id: ${describeIdentity(id)},'
      ' imageName: ${describeIdentity(imageName)},'
      ' scale: $scale'
      ')';
}

// 远端图片
class JM3x4Cover extends StatefulWidget {
  final int comicId;
  final double? width;
  final double? height;
  final BoxFit fit;

  const JM3x4Cover({
    Key? key,
    required this.comicId,
    this.width,
    this.height,
    this.fit = BoxFit.cover,
  }) : super(key: key);

  @override
  State<StatefulWidget> createState() => _JM3x4CoverState();
}

class _JM3x4CoverState extends State<JM3x4Cover> {
  late Future<String> _future;

  @override
  void initState() {
    _future = methods.jm3x4Cover(widget.comicId);
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return pathFutureImage(
      _future,
      widget.width,
      widget.height,
      fit: widget.fit,
      context: context,
    );
  }
}

// 远端图片
class JMSquareCover extends StatefulWidget {
  final int comicId;
  final double? width;
  final double? height;
  final BoxFit fit;

  const JMSquareCover({
    Key? key,
    required this.comicId,
    this.width,
    this.height,
    this.fit = BoxFit.cover,
  }) : super(key: key);

  @override
  State<StatefulWidget> createState() => _JMSquareCoverState();
}

class _JMSquareCoverState extends State<JMSquareCover> {
  late Future<String> _future;

  @override
  void initState() {
    _future = methods.jmSquareCover(widget.comicId);
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return pathFutureImage(
      _future,
      widget.width,
      widget.height,
      fit: widget.fit,
      context: context,
    );
  }
}

class JMPhotoImage extends StatefulWidget {
  final String photoName;

  final double? width;
  final double? height;
  final BoxFit fit;

  const JMPhotoImage({
    Key? key,
    required this.photoName,
    this.width,
    this.height,
    this.fit = BoxFit.cover,
  }) : super(key: key);

  @override
  State<StatefulWidget> createState() => _JMPhotoImageState();
}

class _JMPhotoImageState extends State<JMPhotoImage> {
  late Future<String> _future;

  @override
  void initState() {
    _future = methods.jmPhotoImage(widget.photoName);
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return pathFutureImage(
      _future,
      widget.width,
      widget.height,
      fit: widget.fit,
      context: context,
    );
  }
}

//
class JMPageImage extends StatefulWidget {
  final int id;
  final String imageName;
  final double? width;
  final double? height;
  final Function(Size size)? onTrueSize;

  const JMPageImage(this.id, this.imageName,
      {Key? key, this.width, this.height, this.onTrueSize})
      : super(key: key);

  @override
  State<StatefulWidget> createState() => _JMPageImageState();
}

class _JMPageImageState extends State<JMPageImage> {
  late Future<String> _future;

  @override
  void initState() {
    _future = _init();
    super.initState();
  }

  Future<String> _init() async {
    final _path = await methods.jmPageImage(widget.id, widget.imageName);
    if (widget.onTrueSize != null) {
      ImageSize size = await methods.imageSize(_path);
      widget.onTrueSize!(Size(size.w.toDouble(), size.h.toDouble()));
    }
    return _path;
  }

  @override
  Widget build(BuildContext context) {
    return pathFutureImage(_future, widget.width, widget.height);
  }
}

Widget pathFutureImage(Future<String> future, double? width, double? height,
    {BoxFit fit = BoxFit.cover, BuildContext? context}) {
  return FutureBuilder(
      future: future,
      builder: (BuildContext context, AsyncSnapshot<String> snapshot) {
        if (snapshot.hasError) {
          print("${snapshot.error}");
          print("${snapshot.stackTrace}");
          return buildError(width, height);
        }
        if (snapshot.connectionState != ConnectionState.done) {
          return buildLoading(width, height);
        }
        return buildFile(
          snapshot.data!,
          width,
          height,
          fit: fit,
          context: context,
        );
      });
}

// 通用方法

Widget buildSvg(String source, double? width, double? height,
    {Color? color, double? margin}) {
  var widget = Container(
    width: width,
    height: height,
    padding: margin != null ? EdgeInsets.all(10) : null,
    child: Center(
      child: SvgPicture.asset(
        source,
        width: width,
        height: height,
        color: color,
      ),
    ),
  );
  return GestureDetector(onLongPress: () {}, child: widget);
}

Widget buildMock(double? width, double? height) {
  var widget = Container(
    width: width,
    height: height,
    padding: EdgeInsets.all(10),
    child: Center(
      child: SvgPicture.asset(
        'lib/assets/unknown.svg',
        width: width,
        height: height,
        color: Colors.grey.shade600,
      ),
    ),
  );
  return GestureDetector(onLongPress: () {}, child: widget);
}

Widget buildError(double? width, double? height) {
  return Image(
    image: const AssetImage('lib/assets/error.png'),
    width: width,
    height: height,
  );
}

Widget buildLoading(double? width, double? height) {
  double? size;
  if (width != null && height != null) {
    size = width < height ? width : height;
  }
  return SizedBox(
    width: width,
    height: height,
    child: Center(
      child: Icon(
        Icons.downloading,
        size: size,
        color: Colors.grey.withAlpha(150),
      ),
    ),
  );
}

Widget buildFile(String file, double? width, double? height,
    {BoxFit fit = BoxFit.cover, BuildContext? context}) {
  var image = Image(
    image: FileImage(File(file)),
    width: width,
    height: height,
    errorBuilder: (a, b, c) {
      print("$b");
      print("$c");
      return buildError(width, height);
    },
    fit: fit,
  );
  if (context == null) return image;
  return GestureDetector(
    onLongPress: () async {
      String? choose = await chooseListDialog(
        context,
        title: '请选择',
        values: ['预览图片', '保存图片'],
      );
      switch (choose) {
        case '预览图片':
          Navigator.of(context).push(MaterialPageRoute(
            builder: (context) => FilePhotoViewScreen(file),
          ));
          break;
        case '保存图片':
          saveImageFileToGallery(context, file);
          break;
      }
    },
    child: image,
  );
}
