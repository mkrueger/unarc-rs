program Pack;
uses
  Forms,
  Pmain in 'PMAIN.PAS' {SDIAppForm},
  Pabout in 'PABOUT.PAS' {AboutBox};
{$R *.RES}

begin
  Application.CreateForm(TSDIAppForm, SDIAppForm);
  Application.CreateForm(TAboutBox, AboutBox);
  Application.Run;
end.
 