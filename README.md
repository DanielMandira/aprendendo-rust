1. Cada pedaço de memória no Heap tem exatamente uma variável dona. Quando essa variável sai do escopo (fecha a chave `}`), o Rust libera aquela memória na hora. Isso evita o "Memory Leak" (vazamento de memória) que deixa o PC lento.

2. Você pode ter quantos "leitores" quiser ao mesmo tempo. 

3. Se alguem escrever ou alterar o dado, o Rust só permite UMA pessoa por vez.
    - Se um processo está deletando uma linha e outro está tentando ler a mesma linha no mesmo milissegundo, o sistema trava. O Rust impede isso no compilador.

4. Se não houver espaço sobrando na memoria, o Rust automaticamente procura um espaço maior na RAM, move tudo para lá e apaga o antigo. Tudo isso com segurança!

5. No Rust, erros são tratados como dados. O Rust usa um tipo especial chamado Result. Ele é como uma caixa que pode conter duas coisas:
- OK(Conteudo): Deu tudo certo, aqui está o seu arquivo.
- Err(erro): Algo falhou (arquivo não encontrado, sem permissão, etc).

