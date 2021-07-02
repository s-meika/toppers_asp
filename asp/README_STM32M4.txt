
	TOPPERS/ASP Kernel（Release 1.9.7）STM32F4/L4/WB/DK2
		Toyohashi Open Platform for Embedded Real-Time Systems/
		Advanced Standard Profile Kernel

TOPPERS/ASP STM32M4は以下の10のボードのGCCの開発環境に対応します．
本パッケージは個別パッケージであるため、TOPPERS/ASPカーネルターゲット
非依存部パッケージasp-1.9.3.tar.gzと組み合わせて使用してください．
(1)STM社 STM32F4 Discoveryボード
   Chip:STM32F407VGT6
(2)Olimex社STM32-E407ボード
   Chip:STM32F407ZGT6
(3)日昇テクノロジ社のstm32f429ボード
   Chip:STM32F429
(4)STM社STM32F401 Nucleoボード
   Chip:STM32F401RET6
(5)STM社STM32F446 Nucleo-64ボード
   Chip:STM32F446RET6
(6)STM社STM32F446 Nucleo-144ボード
   Chip:STM32F446ZET6
(7)STM社STM32L476 Nucleo-64ボード
   Chip:STM32L476RGT6
(8)STM社STM32L476 Discoveryボード
   Chip:STM32L476VGT6
(9)STM社STM32L4R5 Nucleo-144ボード
   Chip:STM32L4R5ZIT6
(10)STM社STM32WB55 Nucleoボード
   Chip:STM32WB55RGV6
(11)STM社STM32G474 Nucleo-64ボード
   Chip:STM32G474RET6
(12)STM社STM32G431 Nucleo-64ボード
   Chip:STM32G431RBT6
(13)STM社STM32MP157C-DK2ボード cortexm4
   Chip:STM32M4

ASPの実行形態は以下の２つをサポートします．
実行形態は、コンパイル時の変数DBGENVの設定で変更ができます．
Makefileの設定で変更ができます．

(1)RAM実行：ROMモニタ(rommon)で起動したボードに、UARTを用いて
ASPの実行形式(srec)をダウンロードして実行する形態
rommonのFLASH ROM書込みファイルはtools/rommonに置いてあります．
DBGENVが設定されない場合、またはRAMが設定の場合、この形態のビルドを行います．

(2)ROM実行：FLASH ROMに書き込んで実行する形態
DBGENVにROMが設定の場合、この形態のビルドを行います．

STM社STM32F401 Nucleoボードに関しては、TrueSTUDIOのプロジェクトファイルを
tools/TrueSTUDIOに用意しています．
これにより、直接TrueSTUDIOを用いてビルドが可能です．
実行形態はROM実行に固定です．

【ディレクトリ構成】

arch/arm_m_gcc/common
	cortex-mのコモン部
arch/arm_m_gcc/stm32f4xx
	stm32f4のchip依存部
arch/arm_m_gcc/stm32l4xx
	stm32l4のchip依存部
arch/gcc
	gccの環境部
target/stm32e407_gcc
	STM32-E407ボードのターゲット依存部
target/stm32f4discovery_gcc
	STM32F4 Discoveryボードのターゲット依存部
target/stm32f401nucleo_gcc
	STM32F401 Nucleoボードのターゲット依存部
target/stm32f429board_gcc
	stm32f429ボードのターゲット依存部
target/stm32f446nucleo64_gcc
	stm32f446 Nucleo-64ボードのターゲット依存部
target/stm32f446nucleo144_gcc
	stm32f446 Nucleo-144ボードのターゲット依存部
target/stm32l476nucleo64_gcc
	stm32l476 Nucleo-64ボードのターゲット依存部
target/stm32l476discovery_gcc
	stm32l476 discoveryボードのターゲット依存部
target/stm32l4r5nucleo144_gcc
	stm32l4r5 Nucleo-144ボードのターゲット依存部
target/stm32wb55nucleo_gcc
	stm32wb55 Nucleoボードのターゲット依存部
target/stm32g474nucleo64_gcc
	stm32g474 Nucleo-64ボードのターゲット依存部
target/stm32g431nucleo64_gcc
	stm32g431 Nucleo-64ボードのターゲット依存部
target/stm32mp157cdk2_gcc
	stm32mp157cdk2ボードのcortexM4コアターゲット依存部
tools/rommon
	ROMモニタとUARTの設定手順とROMモニタ書き込みバイナリファイル
	ROMモニタのソースはTOPPERS教育コンテンツ基礎１，２の
	STM32F401-Nucleo 64ボード編にて配布します．
tools/TrueSTUDIO
	STM32F401 Nucleoボード用のTrueSTUDIOのプロジェクトファイル

1.9.x バージョン履歴
	2016年01月03日	Release	1.9.2		最初のリリース
	2016年07月22日	Release	1.9.3		STM32F446 nucleo 64/144ボード対応
	2017年08月28日	Release	1.9.4		STM32L476 ボード対応
	2019年02月04日	Release	1.9.5		STM32L4R5 nucleo 144 ボード対応
	2019年12月19日	Release	1.9.6		STM32WB55 nucleo ボード対応
	                             		STM32G474/STM32G431 nucleo 64  ボード対応
	2021年03月08日	Release	1.9.7		STM32MP157C-DK2 ボード cortexM4コア対応
										STM32WB55 nucleo ボードBLE対応サービスコール追加

