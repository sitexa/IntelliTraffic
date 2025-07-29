## 重要接口

*   **1 /LAPI/V1.0/System/Event/Notification/ObjectRealTimeData**
    *   **功能:** 此接口供设备根据检测帧率推送**实时目标数据**。
    *   **传输协议:** 使用 **WS** 协议。
    *   **说明:** 此接口的输入数据结构包括 `Reference`（用于客户端确认推送事件消息的 URL）、`CurrentTime`（当前时间，精确至毫秒）、`Seq`（序列号）、`DeviceID`（设备 ID）、`TollgateID`（卡口编码）、`SourceID`（数据源 ID）、`SourceType`（数据源类型）、`ObjectNum`（目标数量）以及 `ObjectRealTimeInfoList`（实时目标信息列表）。

*   **2 /LAPI/V1.0/System/Event/Notification/RoadFlow**
    *   **功能:** 此接口用于推送**交通区域统计数据**。
    *   **传输协议:** 使用 **WS** 协议。
    *   **说明:** 输入数据包括 `Reference`, `DeviceID`, `TollgateID`, `SourceID`, `SourceType`, `NotificationType`, `AreaID` 以及包含道路状态信息的 `RoadStatusInfoList` (道路状态信息列表)，例如 `Vehicles`（车辆数）和 `AverageSpeed`（平均速度）。

*   **3 /LAPI/V1.0/System/Event/Notification/TrafficFlow**
    *   **功能:** 此接口用于在约定的周期推送**断面交通流量的统计信息**。
    *   **传输协议:** 使用 **WS** 协议。
    *   **说明:** 输入数据包括 `Reference`, `DeviceID`, `TollgateID`, `CurrentTime`, `Seq`, `SourceID`, `SourceType`, `NotificationType`, `Period`（周期）, `ID`, `LaneNum`（车道数）以及 `LaneFlowInfoList` (车道流量信息列表)，提供每车道的详细交通流量信息。这可以包括 `Volume`（交通量）、`FlowRate`（流率）、`BackOfQueue`（排队长度）、`TravelTime`（旅行时间）以及按类型和车牌颜色分类的各种车辆计数。

*   **4 /LAPI/V1.0/System/Event/Notification/VehicleQueueLen**
    *   **功能:** 此接口用于推送**车辆排队信息**。
    *   **传输协议:** 使用 **WS** 协议。
    *   **说明:** 输入数据包括 `Reference`, `DeviceID`, `TollgateID`, `CurrentTime`, `Seq`, `SourceID`, `SourceType`, `NotificationType` 以及包含排队详细信息的 `VehQueueLenInfo` (车辆排队长度信息)，例如 `QueueLength`（排队长度）、`QueueNum`（排队数量）、`QueueHead`（排队队首）和 `QueueTail`（排队队尾）。

**关键数据表格**

**1. RoadStatusInfo (道路状态信息)**

| 参数名        | 必需性 | 类型        | 描述               |
| ------------- | -------- | ----------- | ------------------ |
| LaneID        | M        | unsigned long | 车道号             |
| Vehicles      | C        | unsigned long | 车辆数             |
| AverageSpeed  | C        | unsigned long | 平均速度           |
| VehicleLength | C        | unsigned long | 平均车外廓长，单位 cm |
| ...           | ...      | ...         | ...                |

**2. LaneFlowInfo (车道流量信息)**

| 参数名              | 必需性 | 类型        | 描述                       |
| ------------------- | -------- | ----------- | -------------------------- |
| LaneID              | M        | unsigned long | 车道号                     |
| Direction           | C        | unsigned long | 车道行驶方向               |
| Vehicles            | C        | unsigned long | 交通量                     |
| AverageSpeed        | C        | unsigned long | 平均速度                   |
| VehicleLength       | C        | unsigned long | 平均车外廓长，单位 cm       |
| TimeOccupyRatio     | C        | unsigned long | 时间占有率，单位：   |
| SpaceHeadway        | C        | unsigned long | 车头间距，单位 厘米/辆       |
| TimeHeadway         | C        | unsigned long | 车头时距，单位 秒/辆         |
| Density             | C        | unsigned long | 车辆密度，单位 辆/km         |
| OverSpeedVehicles   | C        | unsigned long | 超速车辆数                 |
| ...                 | ...      | ...         | ...                        |

**3. VehQueueLenInfo (车辆排队长度信息)**

| 参数名            | 必需性 | 类型        | 描述         |
| ----------------- | -------- | ----------- | ------------ |
| LaneID            | M        | unsigned long | 车道编号     |
| QueueLength       | C        | unsigned long | 排队长度，分米 |
| QueueNum          | C        | unsigned long | 排队数量，辆   |
| QueueHead         | C        | unsigned long | 排队队首，分米 |
| QueueTail         | C        | unsigned long | 排队队尾，分米 |
| ...               | ...      | ...         | ...          |
