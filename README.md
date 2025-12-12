##plug-and-hack puzzle 
Hackeie como um quebra cabeça sem esforço ou curva de aprendizado.

A seguir está um passo a passo para configurar e iniciar o ambiente de testes deste projeto em Rust. O objetivo é permitir que você utilize o código em contextos legítimos, como laboratórios de segurança, CTFs ou ambientes autorizados de pentest.

1. Instalar o Rust

Primeiro, instale o Rust e o Cargo utilizando o instalador oficial:

👉 https://rust-lang.org/tools/install

2. Configurar o Payload

No arquivo src/main.rs, ajuste os parâmetros necessários para o comportamento desejado do payload — como endereço IP e porta do servidor de testes ao qual ele vai se conectar.

// Exemplo (placeholder)
// Ajuste para o ambiente AUTORIZADO de teste
const CALLBACK_IP: &str = "127.0.0.1";
const CALLBACK_PORT: u16 = 4444;


Se quiser tornar o payload interativo, você pode implementar a funcionalidade e abrir um Pull Request para revisão.

3. Definir a API Key do Shodan

Caso o projeto utilize consultas ao Shodan como parte da automação de coleta de informações, inclua sua chave de API válida (e recomendavelmente privada) no mesmo arquivo:

// Preencha com sua própria chave do Shodan
const SHODAN_API_KEY: &str = "<SUA_API_KEY_AQUI>";
