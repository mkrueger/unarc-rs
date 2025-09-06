unit Pmain;

interface

uses WinTypes, WinProcs, Classes, Graphics, Forms, Controls, Menus,
  Dialogs, StdCtrls;

type
  TSDIAppForm = class(TForm)
    MainMenu: TMainMenu;
    FileMenu: TMenuItem;
    OpenItem: TMenuItem;
    SaveItem: TMenuItem;
    ExitItem: TMenuItem;
    N1: TMenuItem;
    OpenDialog: TOpenDialog;
    SaveDialog: TSaveDialog;
    Help1: TMenuItem;
    About1: TMenuItem;
    StatusBar: TPanel;
    procedure ShowHint(Sender: TObject);
    procedure ExitItemClick(Sender: TObject);
    procedure OpenItemClick(Sender: TObject);
    procedure SaveItemClick(Sender: TObject);
    procedure About1Click(Sender: TObject);
    procedure FormCreate(Sender: TObject);
  private
    { Private declarations }
  public
    { Public declarations }
  end;

var
  SDIAppForm: TSDIAppForm;

implementation

uses PAbout, Packer;

{$R *.DFM}

procedure TSDIAppForm.ShowHint(Sender: TObject);
begin
  StatusBar.Caption := Application.Hint;
end;

procedure TSDIAppForm.ExitItemClick(Sender: TObject);
begin
  Close;
end;

procedure TSDIAppForm.OpenItemClick(Sender: TObject);
Var
  aPacker: TPacker;
begin
  aPacker:= TPacker.Create(Self);
  aPacker.Compress;
  if OpenDialog.Execute then
    Caption := Caption+' - ['+OpenDialog.Filename+']';
end;

procedure TSDIAppForm.SaveItemClick(Sender: TObject);
begin
  SaveDialog.Execute;
end;

procedure TSDIAppForm.About1Click(Sender: TObject);
begin
  AboutBox.ShowModal;
end;

procedure TSDIAppForm.FormCreate(Sender: TObject);
begin
  Application.OnHint := ShowHint;
end;

end.
 
