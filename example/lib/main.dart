import 'package:dart_esc_pos_driver/api/printer.dart';
import 'package:dart_esc_pos_driver/dart_esc_pos_driver.dart';
import 'package:flutter/material.dart';

void main() {
  initDartEscPosDriverLibaray();
  runApp(const MyApp());
}

class MyApp extends StatefulWidget {
  const MyApp({super.key});

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  @override
  void initState() {
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(
          title: const Text('Native Packages'),
        ),
        body: ListView(
          children: [
            TextButton(
                onPressed: _getConsolePrinter, child: const Text("连接控制台打印机"))
          ],
        ),
      ),
    );
  }

  Future<void> _getConsolePrinter() async {
    try {
      print('开始连接控制台打印.');
      final consolePrinter = await LddConsoleDriver.open();
      print(consolePrinter);
      final printer = await consolePrinter.openPrinter();
      print(printer);
      await printer.init();
      await printer.align(alignment: LddAlignment.center);
      await printer.qr(
        funCall: () {
          return const DartQrBuilder(size: 200, text: "hello");
        },
      );
      await printer.text(text: "hello 梁典典");
      await printer.feed(n: 4);
      await printer.cut();
      await printer.flush();
      print('结束');
    } catch (e, s) {
      print(e);
      print(s);
    }
  }
}
