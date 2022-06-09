package org.me.javaGUI.container;

import java.awt.*;

public class WindowDemo {
    public static void main(String[] args) {
        //1.创建窗口兑现
        Frame frame = new Frame("这里是测试window窗口");
        //2。指定窗口的大小和位置
        frame.setLocation(100,100);
        frame.setSize(500, 500);
        //3窗口可见
        frame.setVisible(true);
    }
}
