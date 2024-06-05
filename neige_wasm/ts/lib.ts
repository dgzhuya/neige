export function genFontCode(name: string, val: any) {
	console.log('name=', name, ',value=', val)
}

export function logger(info: string, isStart = false) {
	console.log(
		`${isStart ? '' : '\n'}${'_'.repeat(10)}${info}${'_'.repeat(10)}\n`
	)
}
