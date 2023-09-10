mod.rs 
定义了4个常量： 长、宽、背景色、是否低电平 busy


Epd3in27 结构体是3.27 寸墨水屏的直接定义，结构体由DisplayInterface , Color , RefreshLut 三个成员构成，
结构体由DisplayInterface 中定义了通过spi直接操作屏幕ic 的相关方法,如发送指令、发送数据、读busy等，
Color 定义屏幕的背景色
RefreshLut 刷新模式，这个屏好像没有快刷，也没有找到相关资料，暂未处理

command.rs 定义一个枚举包含了所有支持的命令

constants.rs 定义了初始化时的一些常量数据，我不太明白其具体含义

graphics.rs  
Display3in27 此结构体用于绘制相关的操作，由 buffer ， rotation 二个成员构成，并实现了被用于绘制需要实现的相关特性，
    如DrawTarget、OriginDimensions
此文件中还定义了 TwoBitColor 枚举体 、TwoBitColorDisplay 特性 ，TwoBitColorDisplay 中实现了draw_helper方法，
主要的绘制代码就在  draw_helper 中的一系列操作。
绘制的核心思路就是需要按buffer 存储各个像素显示信息的规律进行编码，将各像素按对应的颜色值存入对应数据位中，比如一个像素
只有黑白则只需要一个位存一个像素的值，1为白，0为黑，或反过来即一个byte能保存8个像素的信息。
当一个像素四阶色时就需要两个位，则一个byte只能保存4个像素的信息，相应的lcd 还有一些
rgb565 则需要两个byte 才能保存一个像素的信息，依次类推的将各像素保存到对应数据位。


