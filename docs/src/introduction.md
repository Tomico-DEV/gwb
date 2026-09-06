# Introduction
This is the documentation and design notes for the GWB project, which is a rust implementation of Martti Mäntylä's Geometric Workbench described in [*An Introduction to Solid Modeling*](https://books.google.co.jp/books/about/An_introduction_to_solid_modeling.html?id=CJVRAAAAMAAJ&hl=en&redir_esc=y).

This project is the "predecessor" for the MyCAD 3D CAD software project which is yet to be developed.

# Overview
GWB, short for "Geometric Workbench", is a 3D solid modeler that uses a plane model (boundary-representation or BRep)
for representing solids, and implements it using half-edge data structure. GWB is only capable of polyhedral models,
although the book discusses ways to extend it.