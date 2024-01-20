# Bem-vindo ao Veilid

- [Vista do alto](#vista-do-alto)
- [Rode um nó](#rode-um-nó)
- [Construindo na Veilid](#construindo-na-veilid)
- [Desenvolvimento](#desenvolvimento)

## Vista do alto

A primeira questão a abordar é _“O que é Veilid?”_. A descrição de mais alto nível é que Veilid é uma rede ponto-a-ponto para compartilhar facilmente vários tipos de dados.

Veilid foi pensada com uma dimensão social em mente, para que cada usuário possa ter seu conteúdo pessoal armazenado na rede, mas que também possa compartilhar esse conteúdo com outras pessoas de sua escolha, ou com o mundo inteiro se quiser.

O objetivo principal da rede Veilid é fornecer a infraestrutura para um tipo específico de dados compartilhados: mídias sociais em diversas formas. Isso inclui conteúdo de _"peso"_ leve, como _tweets_ do Twitter/X ou _toots_ do Mastodon, conteúdo médio, como imagens e músicas, e conteúdo pesado, como vídeos. Meta-conteúdo, como _feeds_ pessoais, respostas, mensagens privadas e assim por diante, também devem ser executados na Veilid.

## Rode um nó

A maneira mais fácil de ajudar a expandir a rede Veilid é rodar o seu próprio nó. Cada usuário do Veilid é um nó, mas alguns nós ajudam a rede mais do que outros. Esses nós de suporte da rede são mais pesados do que o nó que um usuário estabeleceria em seu celular na forma de um aplicativo de bate-papo ou mídia social. Um servidor virtual privado (VPS, _virtual private server_) baseado em nuvem, como Digital Ocean Droplets ou AWS EC2, com alta largura de banda, recursos de processamento e disponibilidade de tempo de atividade é crucial para construir o roteamento rápido, seguro e privado que a Veilid foi criada para fornecer.

Para executar tal nó, provisione um VPS baseado em Debian ou Fedora e instale o serviço `veilid-server`. Para simplificar esse processo, estamos hospedando repositórios de gerenciadores de pacotes para pacotes .deb e .rpm. Consulte o guia [instalação](./INSTALL.md) (em Inglês) para obter mais informações.

## Construindo na Veilid

Se você quiser começar a usar a Veilid em seu próprio aplicativo, dê uma olhada no [Livro do Desenvolvedor](https://veilid.gitlab.io/developer-book/) (em Inglês).

Um exemplo básico usando `veilid-core` e `tokio` se parece com o abaixo.

```rust
use std::sync::Arc;
use veilid_core::VeilidUpdate::{AppMessage, Network};
use veilid_core::{VeilidConfigBlockStore, VeilidConfigInner, VeilidConfigProtectedStore, VeilidConfigTableStore, VeilidUpdate};

#[tokio::main]
async fn main() {
    let update_callback = Arc::new(move |update: VeilidUpdate| {
        match update {
            AppMessage(msg) => {
                println!("Message: {}", String::from_utf8_lossy(msg.message().into()));
            }
            Network(msg) => {
                println!("Network: Peers {:}, bytes/sec [{} up] [{} down]", msg.peers.iter().count(), msg.bps_up, msg.bps_down)
            }
            _ => {
                println!("{:?}", update)
            }
        };
    });

    let config = VeilidConfigInner {
        program_name: "Example Veilid".into(),
        namespace: "veilid-example".into(),
        protected_store: VeilidConfigProtectedStore {
            // avoid prompting for password, don't do this in production
            always_use_insecure_storage: true,
            directory: "./.veilid/block_store".into(),
            ..Default::default()
        },
        block_store: VeilidConfigBlockStore {
            directory: "./.veilid/block_store".into(),
            ..Default::default()
        },
        table_store: VeilidConfigTableStore {
            directory: "./.veilid/table_store".into(),
            ..Default::default()
        },
        ..Default::default()
    };

    let veilid = veilid_core::api_startup_config(update_callback, config).await.unwrap();
    println!("Node ID: {}", veilid.config().unwrap().get_veilid_state().config.network.routing_table.node_id);
    veilid.attach().await.unwrap();
    // Until CTRL+C is pressed, keep running
    tokio::signal::ctrl_c().await.unwrap();
    veilid.shutdown().await;
}
```

## Desenvolvimento

Se você deseja se envolver no desenvolvimento de código e não-código, verifique o guia [contribuindo](./CONTRIBUTING.md) (em Inglês). Estamos nos esforçando para que este projeto seja desenvolvido abertamente e por pessoas para pessoas. As áreas específicas nas quais procuramos ajuda incluem:

- Rust
- Flutter/Dart
- Python
- Gitlab DevOps e CI/CD
- Documentação
- Revisões de segurança
- Empacotamento para distros Linux
