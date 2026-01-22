# AeonNode Framework
AeonNode-LoRa 是一個基於 [Embassy.rs](https://embassy.dev/) 異步框架構建的低功耗物聯網開發框架。專為 [RAK3112-9-SM-I](https://docs.rakwireless.com/product-categories/wisduo/rak3112-module/datasheet/) (STM32L0 + SX1276) 設計，旨在提供一個高效、電力自給（太陽能供電）且易於擴展的軟硬件開發底層。

## 🚀 核心特性
* **完全異步驅動 (Async Native)：** 利用 Embassy 的執行器 (Executor)，實現非阻塞式感測器讀取與 LoRa 通訊。
* 極致低功耗管理：
  * 整合 embassy-stm32 的低功耗定時器 (LPTIM)。
  * 自動進入 STOP 模式，僅在外部中斷或定時任務時喚醒。
  * 支援動態時鐘調整，最小化活動狀態電流。
* **模組化感測器抽象：** 預留 I2C/SPI 異步接口，輕鬆整合 BME280、光照計等氣象感測器。
* **太陽能電力監控：** 內置電池電壓與充電狀態的 ADC 採集任務，支持低電量降級運行模式。
* **LoRaWAN 整合：** 針對 SX1276 深度優化的異步狀態機，支持 Class A/C 終端模式。

## 🏗️ 系統架構
```text
┌─────────────────────────────────────────────────────────┐
│                   App Logic (Weather Station)           │
├─────────────────────────────────────────────────────────┤
│            AeonNode Framework (Services & Tasks)        │
│  ┌────────────┐  ┌────────────┐  ┌────────────┐         │
│  │ Power Mgr  │  │ Sensor Hub │  │ LoRa Stack │         │
└──┼────────────┼──┼────────────┼──┼────────────┼─────────┘
   ▼            ▼  ▼            ▼  ▼            ▼
┌─────────────────────────────────────────────────────────┐
│                Embassy.rs HAL (STM32L0)                 │
└─────────────────────────────────────────────────────────┘
          ▲                      ▲                  ▲
    [Solar Panel]          [SX1276 LoRa]     [Environmental Sensors]
```

## 🛠️ 快速開始
### 前置要求
1. 安裝 Rust 工具鏈: rustup target add thumbv6m-none-eabi
2. 安裝 probe-rs 進行燒錄與調試。
3. 準備 RAK3112 硬體或相應的 STM32L0 開發板。

### 編譯與運行
```bash
# 複製專案
git clone git@github.com:CasperHK/AeonNode.git
cd aeonnode

# 運行範例 (預設使用 RAK3112 配置)
cargo run --release --example weather_station
```

## 📂 目錄結構
* `/src/core`: 框架核心，包含異步調度與低功耗策略。
* `/src/drivers`: 感測器驅動（BME280, TSL2591 等）。
* `/src/lora`: LoRaWAN 協議棧封裝。
* `/src/power`: ADC 電池監測與太陽能效率追蹤。
* `/examples`: 完整的參考實現範例。
* `/pcb`: [KiCad](https://www.kicad.org/) 硬件設計檔案。

## 🔋 低功耗設計指標 (預期)
* **Deep Sleep (STOP Mode)：** < 5μA (含 RAK3112 靜態電流)。
* **傳輸瞬間峰值：** ~120mA (20dBm 發射)。
* **平均功耗：** 在每 15 分鐘傳輸一次的情況下，理論續航時間（2500mAh 電池）可達 5 年以上。
* **平均功耗：** 在每分鐘傳輸一次的高頻監測下，系統平均功耗僅約 377μA，僅需微型太陽能板每日充電 10 分鐘，即可在 2500mAh 電池支援下實現永久續航。

## 🤝 貢獻
歡迎提交 Issue 或 Pull Request。針對硬體抽象層的改進與新型感測器的驅動支持是我們目前最需要的。

## 💡 開發建議 (針對 Embassy.rs)
* **使用 static_cell：** 在 Embassy 中定義全域靜態資源（如驅動或緩衝區）時，推薦使用 static_cell 以確保記憶體安全。
* **優先使用 LPTIM：** 對於長達數分鐘的休眠喚醒，請確保配置 embassy-stm32 使用低功耗時鐘源 (LSE/LSI)，否則無法在睡眠模式下喚醒。
* **注意中斷優先級：** RAK3112 的 SX1276 DIO 腳位中斷需要正確分配優先級，以防在高併發採集時丟失數據。

