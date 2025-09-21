(* ::Package:: *)

(* Mathematica example for neopdf_wolfram *)

(* Step 1: Load the library *)
(* Replace with the actual path to the compiled library *)
libPath = "/Users/tanjona/Dropbox/Mac/Documents/Documents/WORK/miscs/neopdf/target/release/libneopdf_wolfram.dylib";

loadPDF = LibraryFunctionLoad[libPath, "NeoPDF_Load", {"UTF8String", Integer}, Integer];
xfxq2 = LibraryFunctionLoad[libPath, "NeoPDF_XFXQ2", {Integer, Integer, {Real, 1}}, Real];
alphasQ2 = LibraryFunctionLoad[libPath, "NeoPDF_AlphasQ2", {Integer, Real}, Real];
clearPDFs = LibraryFunctionLoad[libPath, "NeoPDF_Clear", {}, "Void"];

(* Step 2: Load a PDF set *)
pdfName = "NNPDF40_nnlo_as_01180";
member = 0;
pdfIndex = loadPDF[pdfName, member];
Print["Loaded PDF set ", pdfName, " with member ", member, " at index ", pdfIndex];

(* Step 3: Calculate xfxq2 *)
pid = 21; (* gluon *)
x = 0.01;
q2 = 100.0;
points = {x, q2};
resultXFXQ2 = xfxq2[pdfIndex, pid, points];
Print["xfxq2 for pid=", pid, ", x=", x, ", q2=", q2, " is ", resultXFXQ2];

(* Step 4: Calculate alpha_s *)
q2AlphaS = 100.0;
resultAlphaS = alphasQ2[pdfIndex, q2AlphaS];
Print["alpha_s for q2=", q2AlphaS, " is ", resultAlphaS];

(* Step 5: Clear the loaded PDFs *)
clearPDFs[];
Print["Cleared loaded PDFs."];
