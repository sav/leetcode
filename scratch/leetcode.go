//go:build ignore

package main

import (
	"fmt"
	"reflect"
)

type Example []interface{}

func Assert(fn interface{}, args ...interface{}) {
	expected := args[len(args)-1]

	args = args[:len(args)-1]

	fnValue := reflect.ValueOf(fn)
	argsValues := make([]reflect.Value, len(args))

	for i, arg := range args {
		argsValues[i] = reflect.ValueOf(arg)
	}

	result := fnValue.Call(argsValues)

	if len(result) != 1 {
		panic("Function must return exactly one value")
	}

	actual := result[0].Interface()

	if !reflect.DeepEqual(actual, expected) {
		panic(fmt.Sprintf("test failed: %v: expected=%v, got=%v",
			args, expected, actual))
	}

	fmt.Printf("test passed: %v -> %v\n", args, actual)
}
