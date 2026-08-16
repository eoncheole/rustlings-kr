# 🇰🇷 Rustlings 한국어 (rustlings-kr) 🦀

[Rustlings](https://rustlings.rust-lang.org)의 **한국어 번역** 프로젝트입니다.

Rust 코드를 읽고 쓰는 데 익숙해지기 위한 작은 연습문제들로, [공식 Rust 책](https://doc.rust-lang.org/book)과 병행하여 학습하는 것을 추천합니다.

## 설치 및 실행

### 사전 준비

Rust가 설치되어 있지 않다면 먼저 설치해주세요.

**macOS:**
```bash
xcode-select --install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Linux (Debian/Ubuntu):**
```bash
sudo apt install gcc
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Windows:**

[rust-lang.org/tools/install](https://www.rust-lang.org/tools/install) 에서 설치하세요.

> 설치 후 터미널을 재시작해주세요.

### 1. Rustlings 설치

```bash
cargo install rustlings
```

> **주의:** `rustlings init`은 실행하지 마세요. 영어 기본 버전이 생성됩니다.

### 2. 한국어 버전 받기

```bash
git clone https://github.com/eoncheole/rustlings-kr.git
cd rustlings-kr
```

### 3. 실행

```bash
rustlings
```

## 번역 범위

- ✅ 모든 연습문제 파일 (.rs) - 주석 및 TODO 한국어 번역
- ✅ 모든 섹션 README - 한국어 번역
- ✅ 모든 힌트 (94개) - 한국어 번역
- ✅ 환영 메시지 및 완료 메시지 - 한국어 번역

## 단축키

| 키 | 동작 |
|----|------|
| `h` | 힌트 보기 |
| `n` | 다음 연습문제로 이동 |
| `l` | 연습문제 목록 열기 |
| `r` | 현재 연습문제 다시 실행 |
| `q` | 종료 |

## 참고 링크

- [Rustlings 공식 사이트](https://rustlings.rust-lang.org)
- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [The Rust Programming Language (한국어)](https://doc.rust-kr.org/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings 공식 저장소](https://github.com/rust-lang/rustlings)

## 기여

오역이나 개선 사항을 발견하시면 이슈나 PR을 남겨주세요.

## 라이선스

[MIT License](LICENSE)