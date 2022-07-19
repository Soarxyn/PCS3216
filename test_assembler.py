from sisprog import  print_debug, assemble, link

if __name__ == "__main__":
    assemblyResult = assemble("div.qck", "div.bdc")[1]
    if assemblyResult == "Assembly successful":
        linkingResult = link(["div.bdc"], "div.fita")[1]
        if linkingResult == "Linking successful":
            print_debug("div.fita")
        else:
            print(linkingResult)
    else:
        print(assemblyResult)