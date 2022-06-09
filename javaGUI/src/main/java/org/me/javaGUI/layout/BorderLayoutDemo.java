package org.me.javaGUI.layout;

import java.awt.*;

public class BorderLayoutDemo {
    public static void main(String[] args) {
        Frame frame = new Frame("This is borderLayout test title");

        //1.给frame设置borderLayout布局
        frame.setLayout(new BorderLayout(30,10));
        //2.往borderLayout中添加布局
        frame.add(new Button("北侧按钮"),BorderLayout.NORTH);
        frame.add(new Button("南侧按钮"),BorderLayout.SOUTH);
        frame.add(new Button("东侧按钮"),BorderLayout.EAST);
        frame.add(new Button("西侧按钮"),BorderLayout.WEST);
        frame.add(new Button("中间侧按钮"),BorderLayout.CENTER);
        //3

        frame.pack();
        frame.setVisible(true);
    }
}
