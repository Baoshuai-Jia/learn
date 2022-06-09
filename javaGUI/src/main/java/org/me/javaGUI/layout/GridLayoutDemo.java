package org.me.javaGUI.layout;

import java.awt.*;

public class GridLayoutDemo {
    public static void main(String[] args) {
        Frame frame = new Frame("This is grid layout title(计算器)");

        //1.创建一个Panel对象，里面放一个TextField组件
        Panel panel = new Panel();
        panel.add(new TextField(30));
        //2。 把当前的这个panel放入到frame中的center（frame默认border布局）
        frame.add(panel,BorderLayout.NORTH);
        //3。创建一个panel，设置他的布局为gridLayout
        Panel gridPanel = new Panel();
        gridPanel.setLayout(new GridLayout(3,5, 4,4));
        //4。往panel中添加内容
        for (int i = 0; i < 10; i++) {
            gridPanel.add(new Button(i+""));
        }
        gridPanel.add(new Button("+"));
        gridPanel.add(new Button("-"));
        gridPanel.add(new Button("*"));
        gridPanel.add(new Button("/"));
        gridPanel.add(new Button("."));
        //5。把当前Panel添加到frame中
        frame.add(gridPanel);

        frame.pack();
        frame.setVisible(true);
    }
}
