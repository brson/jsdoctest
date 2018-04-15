/*!
Everything here is safe to click (brson). Anyl local paths work on Win 10.

## javascript links

[js](javascript:alert\(1\))

[reference js]

[reference js]: javascript:alert\(1\)

Case matters:

[upcase js](JAVASCRIPT:alert\(1\))


## local links

[local file](file://C:/Windows/System32/license.rtf)

[reference local file]

[reference local file]: file://C:/Windows/System32/license.rtf

## inline html and scripts

an inline html that invokes a script:

<script type="text/javascript">
function clickme() {
    alert(1);
}
</script>

<a href="#" onclick="clickme()">
click me
</a>

an inline script:

<script type="text/javascript">
document.write("<strong>if you are seeing this it was injected via javascript"</strong>");
</script>

## funky images

js image:

![js image](javascript:alert\(1\))

local file:

![local image](file:///C:/Windows/System32/SecurityAndMaintenance.png)

local text file:

![local text file](file:///C:/Windows/System32/WindowsCodecsRaw.txt)

regular non-local image:

![non-local image](https://i.imgur.com/bHO6PSi.gif)

non-local html served as image:

![non-local html as image](https://gist.githubusercontent.com/brson/45a122f6414877b346932906f70f2901/raw/463f5cc968e8aee67146bc715febc2a93f029a43/foo.hml)

non-local html served as gif

![non-local html served sa gif](https://gist.githubusercontent.com/brson/45a122f6414877b346932906f70f2901/raw/463f5cc968e8aee67146bc715febc2a93f029a43/foo.gif)


!*/

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
