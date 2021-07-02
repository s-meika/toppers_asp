/*
 *  TOPPERS/ASP Kernel
 *      Toyohashi Open Platform for Embedded Real-Time Systems/
 *      Advanced Standard Profile Kernel
 *
 *  Copyright (C) 2015 by Embedded and Real-Time Systems Laboratory
 *              Graduate School of Information Science, Nagoya Univ., JAPAN  
 * 
 *  上記著作権者は，以下の(1)～(4)の条件を満たす場合に限り，本ソフトウェ
 *  ア（本ソフトウェアを改変したものを含む．以下同じ）を使用・複製・改
 *  変・再配布（以下，利用と呼ぶ）することを無償で許諾する．
 *  (1) 本ソフトウェアをソースコードの形で利用する場合には，上記の著作
 *      権表示，この利用条件および下記の無保証規定が，そのままの形でソー
 *      スコード中に含まれていること．
 *  (2) 本ソフトウェアを，ライブラリ形式など，他のソフトウェア開発に使
 *      用できる形で再配布する場合には，再配布に伴うドキュメント（利用
 *      者マニュアルなど）に，上記の著作権表示，この利用条件および下記
 *      の無保証規定を掲載すること．
 *  (3) 本ソフトウェアを，機器に組み込むなど，他のソフトウェア開発に使
 *      用できない形で再配布する場合には，次のいずれかの条件を満たすこ
 *      と．
 *    (a) 再配布に伴うドキュメント（利用者マニュアルなど）に，上記の著
 *        作権表示，この利用条件および下記の無保証規定を掲載すること．
 *    (b) 再配布の形態を，別に定める方法によって，TOPPERSプロジェクトに
 *        報告すること．
 *  (4) 本ソフトウェアの利用により直接的または間接的に生じるいかなる損
 *      害からも，上記著作権者およびTOPPERSプロジェクトを免責すること．
 *      また，本ソフトウェアのユーザまたはエンドユーザからのいかなる理
 *      由に基づく請求からも，上記著作権者およびTOPPERSプロジェクトを
 *      免責すること．
 * 
 *  本ソフトウェアは，無保証で提供されているものである．上記著作権者お
 *  よびTOPPERSプロジェクトは，本ソフトウェアに関して，特定の使用目的
 *  に対する適合性も含めて，いかなる保証も行わない．また，本ソフトウェ
 *  アの利用により直接的または間接的に生じたいかなる損害に関しても，そ
 *  の責任を負わない．
 * 
 *  @(#) $Id: target_config.c 2697 2015-11-06 04:00:42Z ertl-honda $
 */

/*
 * チップ依存モジュール（SAMD51用）
 */
#include "kernel_impl.h"
#include <sil.h>
#include "target_serial.h"

#include "plib_clock.h"

volatile int poi;
extern const FP vector_table[];

/*
 *  start.Sから呼び出されるハードウェアの初期化処理
 */
 __attribute__((weak))
void
hardware_init_hook(void)
{

}


extern void __libc_init_array(void);

/*
 *  start.Sから呼び出されるソフトウェア環境の初期化処理
 */
 __attribute__((weak))
void
software_init_hook(void)
{
//	__libc_init_array();
}

/*
 *  ソフトウェア環境の終了処理
 */
 __attribute__((weak))
void
software_term_hook(void)
{

}

void
port_initialize(void) {
	/* LED Enable */
	PORT_REGS->GROUP[0].PORT_DIRSET = 1 << 15;

	/* LED On */
	PORT_REGS->GROUP[0].PORT_OUTSET = 1 << 15;

	/* for SEMCOM2  */
	PORT_REGS->GROUP[1].PORT_PINCFG[26] = 0x1;
	PORT_REGS->GROUP[1].PORT_PINCFG[27] = 0x41;
	PORT_REGS->GROUP[1].PORT_PMUX[13] = 0x22;
}

void
led_on(void)
{
	PORT_REGS->GROUP[0].PORT_OUTSET = 1 << 15;
}

void
led_off(void)
{
	PORT_REGS->GROUP[0].PORT_OUTCLR = 1 << 15;
}

/*
 *  ターゲット依存の初期化
 */
void
target_initialize(void)
{
	/*
	 *  -fdata-sectionsを使用するとistkが削除され，
	 *  cfgのパス3のチェックがエラーとなるため，
	 *  削除されないようにする 
	 */
	poi = (uint32_t)istk;
	poi = (uint32_t)vector_table[2];

	/*
	 * クロックの初期化
	 */
	CLOCK_Initialize();

	/*
	 *  ポートの初期化
	 */
	port_initialize();

	/*
	 *  target_fput_logが使えるようにUARTを初期化
	 */
	sercom_init_uart(1);
	
	/*
	 *  ARM依存の初期化
	 */
	core_initialize();
}

/*
 *  ターゲット依存の終了処理
 */
void
target_exit(void)
{
	extern void    software_term_hook(void);
	void (*volatile fp)(void) = software_term_hook;

	/*
	 *  software_term_hookへのポインタを，一旦volatile指定のあるfpに代
	 *  入してから使うのは，0との比較が最適化で削除されないようにするた
	 *  めである．
	 */
	if (fp != 0) {
		(*fp)();
	}

	/*
	 *  ARM依存の終了処理
	 */
	core_terminate();

	while(1);
}

/*
 *  システムログの低レベル出力のための文字出力
 */
void
target_fput_log(char c)
{
	if (c == '\n') {
		sercom_putc(SIO_PORTID, '\r');
	}
	sercom_putc(SIO_PORTID, c);
}
