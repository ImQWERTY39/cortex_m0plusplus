	AREA	Reset, DATA, READONLY
	EXPORT	__Vectors

__Vectors
	DCD	0x20001000
	DCD	Reset_Handler

	AREA	|.text|, CODE, READONLY
	THUMB
	EXPORT	Reset_Handler

Reset_Handler PROC
	MOVS		R0, #2
	
	CMP		R0, #1
	BLE		if_73_exit
	MOVS		R4, #1
	B		conditionals_73_exit

if_73_exit
	BGE		elseif_126_exit
	MOVS		R4, #2
	B		conditionals_73_exit
elseif_126_exit

	MOVS		R4, #3

conditionals_73_exit

	PUSH		{R0, R1, R2, R3, R4}
End_Loop
	B		End_Loop
	ENDP

	ALIGN
	END
