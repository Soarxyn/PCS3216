# PCS3216
Projeto da disciplina PCS3216 - Sistemas de Programação, da graduação em Engenharia Elétrica, ênfase de Computação da POLI-USP.

# Instruções
## Setup
Na pasta do projeto, executar:

```
python3 -m venv .env
source .env/bin/activate
python3 -m pip install maturin
```

### Interação com o Ambiente Virtual
Para entrar no ambiente virtual, executar, na pasta do projeto:

```
source .env/bin/activate
```

Para sair do ambiente virtual, executar:

```
deactivate
```

## Build
No ambiente virtual, executar:

```
maturin develop
```

## Execução
No ambiente virtual, executar

```
python3 src/main.py
```
