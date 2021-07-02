/*
 *  TOPPERS/ASP Kernel
 *      Toyohashi Open Platform for Embedded Real-Time Systems/
 *      Advanced Standard Profile Kernel
 * 
 *  Copyright (C) 2008-2011 by Embedded and Real-Time Systems Laboratory
 *              Graduate School of Information Science, Nagoya Univ., JAPAN
 * 
 *  �嵭����Ԥϡ��ʲ���(1)��(4)�ξ������������˸¤ꡤ�ܥ��եȥ���
 *  �����ܥ��եȥ���������Ѥ�����Τ�ޤࡥ�ʲ�Ʊ���ˤ���ѡ�ʣ������
 *  �ѡ������ۡʰʲ������ѤȸƤ֡ˤ��뤳�Ȥ�̵���ǵ������롥
 *  (1) �ܥ��եȥ������򥽡��������ɤη������Ѥ�����ˤϡ��嵭������
 *      ��ɽ�����������Ѿ�浪��Ӳ�����̵�ݾڵ��꤬�����Τޤޤη��ǥ���
 *      ����������˴ޤޤ�Ƥ��뤳�ȡ�
 *  (2) �ܥ��եȥ������򡤥饤�֥������ʤɡ�¾�Υ��եȥ�������ȯ�˻�
 *      �ѤǤ�����Ǻ����ۤ�����ˤϡ������ۤ�ȼ���ɥ�����ȡ�����
 *      �ԥޥ˥奢��ʤɡˤˡ��嵭�����ɽ�����������Ѿ�浪��Ӳ���
 *      ��̵�ݾڵ����Ǻܤ��뤳�ȡ�
 *  (3) �ܥ��եȥ������򡤵�����Ȥ߹���ʤɡ�¾�Υ��եȥ�������ȯ�˻�
 *      �ѤǤ��ʤ����Ǻ����ۤ�����ˤϡ����Τ����줫�ξ�����������
 *      �ȡ�
 *    (a) �����ۤ�ȼ���ɥ�����ȡ����Ѽԥޥ˥奢��ʤɡˤˡ��嵭����
 *        �ɽ�����������Ѿ�浪��Ӳ�����̵�ݾڵ����Ǻܤ��뤳�ȡ�
 *    (b) �����ۤη��֤��̤�������ˡ�ˤ�äơ�TOPPERS�ץ��������Ȥ�
 *        ��𤹤뤳�ȡ�
 *  (4) �ܥ��եȥ����������Ѥˤ��ľ��Ū�ޤ��ϴ���Ū�������뤤���ʤ�»
 *      ������⡤�嵭����Ԥ����TOPPERS�ץ��������Ȥ����դ��뤳�ȡ�
 *      �ޤ����ܥ��եȥ������Υ桼���ޤ��ϥ���ɥ桼������Τ����ʤ���
 *      ͳ�˴�Ť����ᤫ��⡤�嵭����Ԥ����TOPPERS�ץ��������Ȥ�
 *      ���դ��뤳�ȡ�
 * 
 *  �ܥ��եȥ������ϡ�̵�ݾڤ��󶡤���Ƥ����ΤǤ��롥�嵭����Ԥ�
 *  ���TOPPERS�ץ��������Ȥϡ��ܥ��եȥ������˴ؤ��ơ�����λ�����Ū
 *  ���Ф���Ŭ������ޤ�ơ������ʤ��ݾڤ�Ԥ�ʤ����ޤ����ܥ��եȥ���
 *  �������Ѥˤ��ľ��Ū�ޤ��ϴ���Ū�������������ʤ�»���˴ؤ��Ƥ⡤��
 *  ����Ǥ�����ʤ���
 * 
 *  @(#) $Id$
 */

/*
 *		������¸�⥸�塼���ARM-M�ѡ�
 */

#include "kernel_impl.h"
#include "check.h"
#include "task.h"

/*
 *  TOPPERSɸ�����߽�����ǥ�¸��Τ�����ѿ��Ƚ��������
 *  ARMv6-M��ARMv7-M�ǰۤʤ뤿��ifdef���ڤ�ʬ���Ƥ���
 */

#if __TARGET_ARCH_THUMB == 4

volatile bool_t		lock_flag;		/* CPU���å��ե饰���ͤ��ݻ������ѿ� */
volatile uint32_t	saved_iipm;		/* �����ͥ���٥ޥ�������¸�����ѿ� */

static void
init_intmodel(void){
	lock_flag = true;
	saved_iipm = IIPM_ENAALL;
}

#else /* __TARGET_ARCH_THUMB == 3 */

uint32_t ief;			/* IRQ�γ�����׵�ػߥե饰�ξ��� */
uint8_t  ief_systick;	/* SysTick�γ�����׵�ػߥե饰�ξ��� */
uint8_t  iipm;			/* ���ߤγ����ͥ���٥ޥ������� */

static void
init_intmodel(void){
	ief = 0x00;
	ief_systick = 0x00;
	iipm = INT_IPM(0);
}

#endif /* __TARGET_ARCH_THUMB == 4 */

/*
 *  �٥����ơ��֥�(kernel_cfg.c)
 */
extern const FP vector_table[];

/*
 *  �����ƥ��㳰������ߤΡ��㳰�ֹ� 4��15��
 *  �����ͥ��������쥸�����ؤΥ��������Τ��������
 */
static const unsigned int nvic_sys_pri_reg[] = {
	0,
	NVIC_SYS_PRI1,
	NVIC_SYS_PRI2,
	NVIC_SYS_PRI3
};

/*
 *  �㳰�ȳ���ߤγ����ͥ���٤򥻥å�
 *
 *  excno��ARM-M�������Ƥ��� Exception Number ����ꡥ
 */
void
set_exc_int_priority(uint32_t excno, uint32_t nvic_ipm){
	uint32_t tmp, reg;

	/*
	 *  �����ͥ��������쥸�����η���
	 */
	if ((EXCNO_MPU <= excno) && (excno <= IRQNO_SYSTICK)) {
		/*
		 * Exception Number 4(Memory Management)����
		 * Exception Number 15(SysTick)�ޤǤγ����ͥ���٤ϥ����ƥ�ͥ����
		 * �쥸�����ˤ�����ꡥ
		 */
		reg = nvic_sys_pri_reg[excno >> 2];
	}
	else if ((TMIN_INTNO < excno) && (excno <= TMAX_INTNO)){
		/*
		 * IRQ����ߤʤ�
		 */
		reg = NVIC_PRI0 + (((excno - (TMIN_INTNO + 1)) >> 2) * 4);
	}
	else {
		return ;
	}

	tmp = sil_rew_mem((void *)reg);
	tmp &= ~(0xFF << (8 * (excno & 0x03)));
	tmp |= nvic_ipm << (8 * (excno & 0x03));
	sil_wrw_mem((void *)reg, tmp);
}

/*
 *  �㳰�ε���
 *
 *  Memory Management, Bus Fault, Usage Fault �϶ػߡ����Ĥ���ǽ
 */
void
enable_exc(EXCNO excno)
{
	uint32_t tmp;

	switch (excno) {
	  case EXCNO_MPU:
		tmp = sil_rew_mem((void *)NVIC_SYS_HND_CTRL);
		tmp |= NVIC_SYS_HND_CTRL_MEM;
		sil_wrw_mem((void *)NVIC_SYS_HND_CTRL, tmp);
		break;
	  case EXCNO_BUS:
		tmp = sil_rew_mem((void *)NVIC_SYS_HND_CTRL);
		tmp |= NVIC_SYS_HND_CTRL_BUS;
		sil_wrw_mem((void *)NVIC_SYS_HND_CTRL, tmp);
		break;
	  case EXCNO_USAGE:
		tmp = sil_rew_mem((void *)NVIC_SYS_HND_CTRL);
		tmp |= NVIC_SYS_HND_CTRL_USAGE;
		sil_wrw_mem((void *)NVIC_SYS_HND_CTRL, tmp);
		break;
	}
}

/*
 *  �㳰�ζػ�
 */
void
disable_exc(EXCNO excno)
{
	uint32_t tmp;

	switch (excno) {
	  case EXCNO_MPU:
		tmp = sil_rew_mem((void *)NVIC_SYS_HND_CTRL);
		tmp &= ~NVIC_SYS_HND_CTRL_MEM;
		sil_wrw_mem((void *)NVIC_SYS_HND_CTRL, tmp);
		break;
	  case EXCNO_BUS:
		tmp = sil_rew_mem((void *)NVIC_SYS_HND_CTRL);
		tmp &= ~NVIC_SYS_HND_CTRL_BUS;
		sil_wrw_mem((void *)NVIC_SYS_HND_CTRL, tmp);
		break;
	  case EXCNO_USAGE:
		tmp = sil_rew_mem((void *)NVIC_SYS_HND_CTRL);
		tmp &= ~NVIC_SYS_HND_CTRL_USAGE;
		sil_wrw_mem((void *)NVIC_SYS_HND_CTRL, tmp);
		break;
	}
}


/*
 *  ������¸�ν����
 */
void
core_initialize(void)
{
	/*
	 *  �٥����ơ��֥������
	 */
	sil_wrw_mem((void*)NVIC_VECTTBL, (uint32_t)vector_table);

	/*
	 *  ���㳰��ͥ���٤�����
	 *  CPU���å����֤Ǥ�ȯ������褦�ˡ�BASEPRI�쥸�����ǥޥ����Ǥ�
	 *  �ʤ�'0'�Ȥ��롥
	 */
	set_exc_int_priority(EXCNO_HARD, 0);
	set_exc_int_priority(EXCNO_MPU, 0);
	set_exc_int_priority(EXCNO_BUS, 0);
	set_exc_int_priority(EXCNO_USAGE, 0);
	set_exc_int_priority(EXCNO_SVCALL, 0);
	set_exc_int_priority(EXCNO_DEBUG, 0);
	set_exc_int_priority(EXCNO_PENDSV, 0);

	/*
	 *  ����߽�����ǥ��Ϣ�ν����
	 */
	init_intmodel();
}

/*
 *  ������¸�ν�λ����
 */
void
core_terminate(void)
{
	extern void    software_term_hook(void);
	void (*volatile fp)(void) = software_term_hook;

	/*
	 *  software_term_hook�ؤΥݥ��󥿤򡤰�övolatile����Τ���fp����
	 *  �����Ƥ���Ȥ��Τϡ�0�Ȥ���Ӥ���Ŭ���Ǻ������ʤ��褦�ˤ��뤿
	 *  ��Ǥ��롥
	 */
	if (fp != 0) {
		(*fp)();
	}
}

/*
 *  ������׵�饤��°��������
 */
void
x_config_int(INTNO intno, ATR intatr, PRI intpri)
{
	assert(VALID_INTNO_CFGINT(intno));
	assert(TMIN_INTPRI <= intpri && intpri <= TMAX_INTPRI);

	/* 
	 *  ��ö����ߤ�ػߤ���
	 */    
	(void)x_disable_int(intno);

	/*
	 *  �����ͥ���٤򥻥å�
	 */
	set_exc_int_priority(intno, INT_NVIC_PRI(intpri));

	/*
	 *  ������׵�ޥ������(ɬ�פʾ��)
	 */
	if ((intatr & TA_ENAINT) != 0U) {
		(void) x_enable_int(intno);
	}    
}


#ifndef OMIT_DEFAULT_EXC_HANDLER
/*
 *  ̤������㳰��ȯ������ȸƤӽФ����
 */
void
default_exc_handler(void *p_excinf)
{
	uint32_t basepri = *(((uint32_t*)p_excinf) + P_EXCINF_OFFSET_IIPM);
	uint32_t pc      = *(((uint32_t*)p_excinf) + P_EXCINF_OFFSET_PC);
	uint32_t xpsr    = *(((uint32_t*)p_excinf) + P_EXCINF_OFFSET_XPSR);
	uint32_t excno   = get_ipsr() & IPSR_ISR_NUMBER;

	syslog(LOG_EMERG, "\n Exception occurs.");
	syslog(LOG_EMERG, "Excno = 0x%08X, PC = 0x%08X, XPSR = 0x%08X, iipm = 0x%08X, p_excinf = 0x%08X",
		   excno, pc, xpsr, basepri, p_excinf);	

	target_exit();
}
#endif /* OMIT_DEFAULT_EXC_HANDLER */

#ifndef OMIT_DEFAULT_INT_HANDLER
/*
 *  ̤��Ͽ�γ���ߤ�ȯ���������˸ƤӽФ����
 */
void
default_int_handler(void *p_excinf)
{
	uint32_t basepri = *(((uint32_t*)p_excinf) + P_EXCINF_OFFSET_IIPM);
	uint32_t pc      = *(((uint32_t*)p_excinf) + P_EXCINF_OFFSET_PC);
	uint32_t xpsr    = *(((uint32_t*)p_excinf) + P_EXCINF_OFFSET_XPSR);
	uint32_t excno   = get_ipsr() & IPSR_ISR_NUMBER;

	syslog(LOG_EMERG, "\nUnregistered Interrupt occurs.");
	syslog(LOG_EMERG, "Excno = 0x%08X, PC = 0x%08X, XPSR = 0x%08X, iipm = 0x%08X, p_excinf = 0x%08X",
		   excno, pc, xpsr, basepri, p_excinf);	

	target_exit();
}
#endif /* OMIT_DEFAULT_INT_HANDLER */
