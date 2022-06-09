package org.me.javaGUI.container;

import java.awt.*;

public class ScrollPanelDemo {
    public static void main(String[] args) {
        Frame frame = new Frame("ScrollPanel");

        //1创建scrollPanel
        ScrollPane scrollPane = new ScrollPane(ScrollPane.SCROLLBARS_ALWAYS);
        //2 往scrollPanel中添加内容
        scrollPane.add(new TextField("This is test text"));
        scrollPane.add(new Button("button"));
        //3。 把scrollPanel添加到window
        frame.add(scrollPane);

        frame.setSize(500,500);
        frame.setVisible(true);
    }
}
