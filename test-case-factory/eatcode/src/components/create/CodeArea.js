import CodeEditor from '@uiw/react-textarea-code-editor';
import Axios from 'axios'
import Button from '../common/Button'
import { useContext, useState } from 'react'
import { colors } from '../../global/vars'
import TestResultBar from './TestResultBar'
import Select from 'react-select'

import { userContext } from '../../userContext';

export default function CodeArea({ color, questionID, userSolvedThis, beef }) {
  const user = useContext(userContext)
  const [getCookingText, setGetCookingText] = useState("get cookin")
  const [code, setCode] = useState(`#receieve test case from standard input\ninput1 = input()\ninput2 = input()\n\n#compute solution\ndef solve(num1, num2):\n  ans = num1 + num2\n  return ans\n  \n#print output\nprint(solve(input1, input2))`);
  const [result, setResult] = useState([]);
  const [lang, selectedLang] = useState("py")

  let languageOptions = [
    { value: "cpp", label: "c++"}, { value: "java", label: "java"}, { value: "py", label: "python"}
  ]

  function handleLanguageChange(e) {
    selectedLang(e.value)
  }

  const handleSubmit = () => {
    if (user.user.userID == null) {
      alert("You must be signed in to submit a problem.")
    } else {
      console.log("Submitted Problem");
      setGetCookingText("cooking...")
      Axios.post("http://localhost:3002/problems", {
        code: code, 
        language: lang, 
        questionID: questionID,
        userID: user.user.userID
      }).then((response) => {
        let result = response.data.result
        setResult(response.data.result);
        if (!result.includes(0)) {
          setGetCookingText("try again?")
        } else {
          setGetCookingText("success!")
        }
      });
    }
  }

  const styles = {
    container: {
      display: 'flex',
      flexDirection: 'column',
      gap: '1em',
      height: '100%'
    },
    alreadySolved: {
      display: userSolvedThis ? 'inline' : 'none',
      backgroundColor: colors.accent1,
      padding: '0.4em',
      textAlign: 'center',
      fontWeight: 'bold',
      color: 'white',
      marginBottom: '0em',
      letterSpacing: '0.5px'
    },
    codeEditor: {
      fontSize: '1.7em',
      backgroundColor: colors.codeBg,
      fontFamily: 'ui-monospace,SFMono-Regular,SF Mono,Consolas,Liberation Mono,Menlo,monospace',
      minHeight: '50%',
      maxHeight: '50%',
      overflowY: 'scroll',
      color: 'white'
    },
    buttonDiv: {
      display: "flex",
      width: '100%',
      gap: '2em',
      justifyContent: 'space-between',
      alignItems: 'center'
    },
    testGrid: {
      display: 'grid',
      gridTemplateColumns: result.length == 0 ? '1fr' : '1fr 1fr',
      gap: '1.5em'
    },
    noSubmission: {
      backgroundColor: "#333333",
      width: '100%',
      display: 'flex',
      alignItems: 'center',
      justifyContent: 'center',
      padding: '5em 0em',
      color: colors.white
    },
    langSelect: {
      container: (styles) => ({
        ...styles,
        height: '100%',
        flex: 1
      }),
      control: (styles, state) => ({
        ...styles,
        color: colors.white,
        height: 'calc(100%)',
        textAlign: 'center',
        fontSize: '3em',
        "&:hover": {
          backgroundColor: colors.black
        },
        transition: 'all 0.3s ease',
        fontWeight: 'bold',
        border: 'none',
        outline: 'none',
        outlineColor: 'transparent'
      }),
      option: (styles, state) => {
        return {
          ...styles,
          color: colors.white,
          fontWeight: 'bold',
          fontSize: '2em',
          transition: 'all 0.3s ease'
        }
      },
      singleValue: (styles) => {
        return {
          ...styles,
          color: colors.white
        }
      },
      placeholder: (styles) => {
        return {
          ...styles,
          color: colors.white
        }
      },
      input: (styles) => {
        return {
          ...styles,
          color: colors.white,
        }
      },
      noOptionsMessage: (styles) => {
        return { 
          ...styles,
          color: colors.black,
          fontWeight: 'bold',
          fontSize: '2em',
          noOptionsText: "tag not found"
        }
      },
      groupHeading: (styles) => {
        return {
          ...styles,
          textTransform: 'none',
          color: colors.accent1,
          fontWeight: 'bold',
          fontSize: '2em',
          textAlign: 'center'
        }
      }
    }
  }


  return (
  <div style={styles.container}>
    <p style={styles.alreadySolved}>You have already solved this problem and gained {beef} 🥩.</p>
    <CodeEditor
      value={code}
      language={lang}
      placeholder=""
      onChange={(evn) => setCode(evn.target.value)}
      style={styles.codeEditor}
    />
    <div style={styles.buttonDiv}>
      <Button onClick={handleSubmit} color={color} text={getCookingText}/>
      <Select styles={styles.langSelect} options={languageOptions} onChange={handleLanguageChange} closeMenuOnSelect defaultValue={languageOptions[2]} noOptionsMessage={() => "tag not found"}
          theme={(theme) => ({
            ...theme,
            borderRadius: 0,
            colors: {
              ...theme.colors,
              primary25: colors.black,
              primary: "#333333", //same color as default test cases box
              neutral0: colors.accent1,
              neutral20: colors.white
            },
        })}/>
    </div>
    <div style={styles.testGrid}>

      {result.length == 0
      ? <div style={styles.noSubmission}><h4>submit to see results</h4></div>
      : result.map((test, i) => {
          return (
            <TestResultBar key={i} number={i+1} code={test} />
          )
          })
      }

    </div>
    </div>
  );
}