package org.me.javaGUI.container;

import java.awt.*;

public class PanelDemo {
    public static void main(String[] args) {
        //1。 创建一个window对象，因为panel及其他容器不可以独立存在，必须依赖window
        Frame frame = new Frame("panel的标题");
        //2。设置window的位置和大小
        frame.setLocation(100,100);
        frame.setSize(500, 500);
        //3.创建一个Panel对象
        Panel panel = new Panel();
        //4。往panel中添加元素（按钮和文本框）
        panel.add(new TextField("这是一个测试文本框"));
        panel.add(new Button("测试"));
        //5。把panel放入到window
        frame.add(panel);
        //6。设置window可见
        frame.setVisible(true);
    }
}
