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
loopstwice
	SUBS		R0, R0, #1
	BNE		loopstwice
	NOP
End_Loop
	B		End_Loop
	ENDP

	ALIGN
	END
