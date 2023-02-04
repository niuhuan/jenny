cd "$( cd "$( dirname "$0"  )" && pwd  )/.."

echo $KEY_FILE_BASE64 > key.jks.base64
base64 -d key.jks.base64 > key.jks
echo $KEY_PASSWORD | $ANDROID_HOME/build-tools/30.0.2/apksigner sign --ks key.jks build/app/outputs/flutter-apk/app-release.apk
