import 'package:jenny/basic/methods.dart';

int _androidVersion = 0;

int get androidVersion => _androidVersion;

Future initAndroidVersion() async {
  _androidVersion = await methods.androidGetVersion();
}
