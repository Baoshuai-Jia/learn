package org.me.javaGUI.layout;

import java.awt.*;

public class FlowLayoutDemo {
    public static void main(String[] args) {
        Frame frame = new Frame("This is flowLayout test");
        //1.通过setLayout方法设置容器的布局管理器
//        frame.setLayout(new FlowLayout(FlowLayout.LEFT,20,20));//左对齐
        frame.setLayout(new FlowLayout(FlowLayout.CENTER,20,20));//居中对齐
        //2。添加多个按钮到frame中
        for (int i = 1; i <= 100; i++) {
            frame.add(new Button("testButton-"+i));
        }
        //3。设置最佳大小(宽度)， pack方法
        frame.pack();
        frame.setVisible(true);
    }
}
