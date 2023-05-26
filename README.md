# Unnamed

WIP

## To Do

- [x] Lexer
- [x] Parser
- [ ] Safe wrapper over LLVM
- [ ] Compiler 

## Function declaration
```ts
func add(a: int, b: int) : int {
    a + b
}
```
return type is optional
```ts
func foo() {
    print("bar");
}
```

## Variable declaration
```ts
let foo
```
```ts
let bar = 10
```
```ts
let mut foo
```
```ts
let mut bar = 10
```

## If expression
```ts
if foo > bar {} else if foo < bar {} else {}
```

```ts
print(if foo == bar { 42 } else { 0 })
```

## While expression

```ts
while i < 10 {}
```

```ts
print(while i < 10 { 0 })
```